use std::{
    f32::{consts::PI, INFINITY},
    time::Duration,
};

use bevy::{input::mouse::MouseButtonInput, math::Vec2Swizzles, prelude::*, window::PrimaryWindow};
use bevy_tweening::{
    lens::{TransformRotationLens, TransformScaleLens},
    *,
};
use leafwing_input_manager::{
    action_state,
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use super::{
    card::{Card, FlipCard, Flipping},
    deck::{draw_card, Deck, Discard},
    rules::{AddRule, Rule},
    Actions, GameState,
};
use crate::{
    board::{self, config},
    camera::{lerp, CardCamera},
    utils::{calculate_rotated_bounds, point_in_board, point_in_polygon},
    AppState,
};

#[derive(Component)]
pub struct Hand {
    pub selected: Option<Entity>,
    pub hovered: Option<Entity>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformLens {
    /// Start value.
    pub start: Transform,
    /// End value.
    pub end: Transform,
}

impl Lens<Transform> for TransformLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        //rotation
        target.rotation = self.start.rotation.slerp(self.end.rotation, ratio);
        //position
        let value =
            self.start.translation + (self.end.translation - self.start.translation) * ratio;
        target.scale = self.start.scale.lerp(self.end.scale, ratio);

        target.translation = value;
    }
}

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_hand)
            .add_systems(Update, component_animator_system::<Transform>)
            .add_systems(
                Update,
                (
                    position_cards.before(draw_card),
                    (pickable_lerp, select_card).run_if(in_state(GameState::Playing)),
                )
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

//spawn deck when deck plugin is made
fn spawn_hand(mut commands: Commands) {
    commands
        .spawn(
            (SpatialBundle {
                transform: Transform::from_xyz(0., -(board::config::SIZE.y + 190. + 50.) / 2., 0.),
                ..Default::default()
            }),
        )
        .insert(Hand {
            selected: None,
            hovered: None,
        });
}
//whenever hand is updated position cards in hand that are not selected by ord using a tween
fn position_cards(
    mut cmd: Commands,
    q_hand: Query<(&Hand, &Children)>,
    mut q_cards: Query<(Entity, &Card, &mut Transform)>,
    mut q_flipping: Query<&Flipping>,
) {
    if q_hand.is_empty() {
        return;
    }

    let (hand, children) = q_hand.single();
    let hand_size = children.len();
    let arc_length = 180.0;
    let rotation_factor = 30.; // Adjust the rotation factor as desired

    let width = (hand_size * 80).clamp(0, 600);

    for (i, &child) in children.iter().enumerate() {
        if let Ok((entity, card, mut transform)) = q_cards.get_mut(child) {
            if hand.selected == Some(entity) {
                continue;
            }

            let angle = (i as f32 / (hand_size as f32)) * arc_length;
            let x = i as f32 / hand_size as f32 * width as f32 - 300.;
            let y = angle.to_radians().sin() * 40.0; // Calculate y position along the arc

            let mut rot = i as f32 / hand_size as f32 * rotation_factor - rotation_factor / 2.;
            if !card.face_up {
                rot *= -1.;
            } else {
                rot += 180.;
            }
            transform.translation.x = transform.translation.x.lerp(&x, &0.2);
            transform.translation.y = transform.translation.y.lerp(&y, &0.2);
            transform.translation.z = i as f32 * 10.;
            if !q_flipping.contains(entity) {
                transform.rotation = transform.rotation.lerp(
                    Quat::from_euler(EulerRot::XYZ, PI, 0., rot.to_radians()),
                    0.2,
                );
            }
        }
    }
}
//whenever a card is selected move it toward the target
fn pickable_lerp(
    mut q_hand: Query<(&Hand, &Transform), (Without<Card>)>,
    mut q_cards: Query<(Entity, &Card, &mut Transform)>,
    mut q_camera: Query<(&Camera, &GlobalTransform), With<CardCamera>>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((hand_comp, hand_transform)) = q_hand.get_single() {
        let Some(selected) = hand_comp.selected else {
            return;
        };

        if let Some(pos) = q_window.single().cursor_position() {
            let (camera, camera_transform) = q_camera.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                let world_pos = world_pos
                    - Vec2::new(hand_transform.translation.x, hand_transform.translation.y);

                if let Ok((entity, card, mut transform)) = q_cards.get_mut(selected) {
                    transform.translation.x = transform.translation.x.lerp(&world_pos.x, &0.2);
                    transform.translation.y = transform.translation.y.lerp(&world_pos.y, &0.2);
                    transform.translation.z = 100.
                }
            }
        }
    }
}

fn select_card(
    mut cmd: Commands,
    mut actions: Query<&ActionState<Actions>>,
    mut q_hand: Query<(&mut Hand, &mut Children, &Transform)>,
    mut q_window: Query<&Window, (With<PrimaryWindow>, Without<Discard>)>,
    mut q_cards: Query<(Entity, &Card, &mut Transform), Without<Hand>>,
    mut q_camera: Query<(&Camera, &GlobalTransform), With<CardCamera>>,
    mut q_rules: Query<(Entity, &Transform), (With<Rule>, Without<Card>)>,
    mut add_rule: EventWriter<AddRule>,
) {
    if q_hand.is_empty() {
        return;
    }

    let (mut hand, children, hand_transform) = q_hand.single_mut();
    let action_state = actions.single();
    let mut hovered_entity = None;

    if hand.selected.is_none() {
        if let Some(pos) = q_window.single().cursor_position() {
            let (camera, camera_transform) = q_camera.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                for &child in children.iter() {
                    //get the topmost hovered card
                    if let Ok((entity, card, transform)) = q_cards.get_mut(child) {
                        //card is 140,190
                        let half_width = 70.;
                        let half_height = 95.;
                        let rotated_bounds = calculate_rotated_bounds(
                            &transform,
                            half_width,
                            half_height,
                        )
                        .map(|corner| {
                            Vec2::new(hand_transform.translation.x, hand_transform.translation.y)
                                + corner
                        });

                        if point_in_polygon(world_pos, &rotated_bounds) {
                            hovered_entity = Some(entity);
                        }
                    }
                }
            }
        }
        if hovered_entity != hand.hovered {
            if let Some(h) = hand.hovered {
                if let Ok((entity, card, transform)) = q_cards.get(h) {
                    let tween = Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_millis(100),
                        TransformScaleLens {
                            start: transform.scale,
                            end: Vec3::new(1., 1., 1.),
                        },
                    );

                    cmd.entity(hand.hovered.unwrap())
                        .insert(Animator::new(tween));
                }
            }

            hand.hovered = hovered_entity;
            if let Some(h) = hand.hovered {
                if let Ok((entity, card, transform)) = q_cards.get(h) {
                    let tween = Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_millis(100),
                        TransformScaleLens {
                            start: transform.scale,
                            end: Vec3::new(1.1, 1.1, 1.),
                        },
                    );
                    cmd.entity(entity).insert(Animator::new(tween));
                }
            }
        }
        // if action_state.just_pressed(CardAction::Flip) && hand.hovered.is_some() {
        //     flip_writer.send(FlipCard {
        //         card: hand.hovered.unwrap(),
        //     });
        // }
        if action_state.just_pressed(Actions::Select) && hand.hovered.is_some() {
            hand.selected = hand.hovered;

            if let Ok((entity, card, transform)) = q_cards.get(hand.selected.unwrap()) {
                //straigten the card
                let before = transform.rotation.to_euler(EulerRot::XYZ);
                let mut rot: f32 = 0.;
                if card.face_up {
                    rot = 180.;
                }
                let tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(250),
                    TransformRotationLens {
                        start: transform.rotation,
                        end: Quat::from_euler(EulerRot::XYZ, before.0, before.1, rot.to_radians()),
                    },
                );
                cmd.entity(entity).insert(Animator::new(tween));
            }
        }
    }

    let select_released = action_state.just_released(Actions::Select);
    if select_released && hand.selected.is_some() {
        if let Ok((entity, card, mut card_transform)) = q_cards.get_mut(hand.selected.unwrap()) {
            let g_x = card_transform.translation.x + hand_transform.translation.x;
            let g_y = card_transform.translation.y + hand_transform.translation.y;
            if point_in_board(g_x, g_y, config::SIZE, config::CENTER) {
                let (rules_e, rules_t) = q_rules.single();

                cmd.entity(entity).remove_parent();
                card_transform.translation.x +=
                    -rules_t.translation.x + hand_transform.translation.x;
                card_transform.translation.y +=
                    -rules_t.translation.y + hand_transform.translation.y;

                cmd.entity(rules_e).insert_children(0, &[entity]);

                cmd.insert_resource(NextState(Some(GameState::Discard)));
                add_rule.send(AddRule {
                    rule: card.operation.clone(),
                });
            }
        }

        hand.selected = None;
    }
}
