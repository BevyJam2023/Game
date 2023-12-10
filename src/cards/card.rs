use std::f32::consts::PI;

use bevy::{ecs::event::EventId, prelude::*};
use leafwing_input_manager::{prelude::InputManagerPlugin, Actionlike};

use super::CardAction;
use crate::{
    game_shapes::{ColorMaterialAssets, Shape, ShapeAssets},
    goal::Goal,
    loading::TextureAssets,
    operation::Operation,
    AppState,
};

#[derive(Component)]
pub struct Card {
    pub front: Entity,
    pub back: Entity,
    pub face_up: bool,
    pub operation: Operation,
}
#[derive(Event)]
pub struct FlipCard {
    pub card: Entity,
}
#[derive(Event)]
pub struct SpawnCard {
    pub zone_id: Entity,
    pub operation: Operation,
    pub face_up: bool,
}
#[derive(Event)]
pub struct SpawnGoalCard {
    pub zone_id: Entity,
    pub goal: Goal,
    pub face_up: bool,
}

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub sprite: SpriteBundle,
}
#[derive(Component)]
pub struct CardFace {
    pub is_front: bool,
}

#[derive(Component)]
pub struct Flipping {
    half: bool,
    rotation_speed: f32,
    current_rotation: f32,
}

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (flip_card, spawn_card, spawn_goal_card).run_if(in_state(AppState::Playing)),
        )
        .add_event::<SpawnGoalCard>()
        .add_event::<FlipCard>()
        .add_event::<SpawnCard>();
    }
}
fn spawn_goal_card(
    mut cmd: Commands,
    mut reader: EventReader<SpawnGoalCard>,
    textures: Res<TextureAssets>,
    ma: Res<ShapeAssets>,
    c_m: Res<ColorMaterialAssets>,
) {
    for event in reader.read() {
        let operation_entity = event.goal.get_goal_entity(&mut cmd, &textures, &ma, &c_m);
        let front = cmd
            .spawn((
                SpriteBundle {
                    texture: textures.card_blank.clone(),
                    visibility: if event.face_up {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    },
                    transform: Transform {
                        rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
                        ..default()
                    },

                    ..default()
                },
                CardFace { is_front: true },
            ))
            .id();
        let back = cmd
            .spawn((
                SpriteBundle {
                    visibility: if event.face_up {
                        Visibility::Hidden
                    } else {
                        Visibility::Inherited
                    },
                    texture: textures.card_blue.clone(),
                    ..default()
                },
                CardFace { is_front: false },
            ))
            .id();

        let card_id = cmd
            .spawn(CardBundle {
                card: Card {
                    back,
                    front,
                    face_up: event.face_up,
                    operation: Operation::None,
                },
                sprite: SpriteBundle { ..default() },
            })
            .id();
        cmd.entity(front).push_children(&operation_entity);

        cmd.entity(card_id).push_children(&[front, back]);
        cmd.entity(event.zone_id).push_children(&[card_id]);
    }
}

fn spawn_card(
    mut cmd: Commands,
    mut reader: EventReader<SpawnCard>,
    textures: Res<TextureAssets>,
    ma: Res<ShapeAssets>,
    c_m: Res<ColorMaterialAssets>,
) {
    for event in reader.read() {
        let operation_entity = event
            .operation
            .get_operation_entity(&mut cmd, &textures, &ma, &c_m);
        let front = cmd
            .spawn((
                SpriteBundle {
                    texture: textures.card_blank.clone(),
                    visibility: if event.face_up {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    },
                    transform: Transform {
                        rotation: Quat::from_euler(EulerRot::XYZ, 0., PI, 0.),
                        ..default()
                    },

                    ..default()
                },
                CardFace { is_front: true },
            ))
            .id();
        let back = cmd
            .spawn((
                SpriteBundle {
                    visibility: if event.face_up {
                        Visibility::Hidden
                    } else {
                        Visibility::Inherited
                    },
                    texture: textures.card_blue.clone(),
                    ..default()
                },
                CardFace { is_front: false },
            ))
            .id();

        let card_id = cmd
            .spawn(CardBundle {
                card: Card {
                    back,
                    front,
                    face_up: event.face_up,
                    operation: event.operation.clone(),
                },
                sprite: SpriteBundle { ..default() },
            })
            .id();
        cmd.entity(front).push_children(&operation_entity);

        cmd.entity(card_id).push_children(&[front, back]);
        cmd.entity(event.zone_id).push_children(&[card_id]);
    }
}

//TODO rotate in axis of rotation so the card flips not in y unless straight
pub fn flip_card(
    mut q_cards: Query<(Entity, &mut Card), Without<Flipping>>,
    mut q_flipping: Query<(Entity, &mut Card, &mut Flipping, &mut Transform)>,
    mut q_faces: Query<(&CardFace, &mut Visibility)>,
    mut flip_event: EventReader<FlipCard>,
    mut cmd: Commands,
    time: Res<Time>,
) {
    for e in flip_event.read() {
        if let Ok((entity, mut card)) = q_cards.get_mut(e.card) {
            card.face_up = !card.face_up;
            cmd.entity(entity).insert(Flipping {
                half: false,
                rotation_speed: 400.0,
                current_rotation: 0.0,
            });
        }
    }
    for (entity, mut card, mut flipping, mut transform) in q_flipping.iter_mut() {
        let rotation_angle = flipping.rotation_speed * time.delta_seconds();
        flipping.current_rotation += rotation_angle;
        if flipping.current_rotation > 90. && !flipping.half {
            flipping.half = true;

            if let Ok((front, mut f_vis)) = q_faces.get_mut(card.front) {
                match card.face_up {
                    true => *f_vis = Visibility::Visible,
                    false => *f_vis = Visibility::Hidden,
                }
            }
            if let Ok((back, mut b_vis)) = q_faces.get_mut(card.back) {
                match card.face_up {
                    true => *b_vis = Visibility::Hidden,
                    false => *b_vis = Visibility::Visible,
                }
            }
        }
        if flipping.current_rotation >= 180.0 {
            flipping.current_rotation = 0.;

            cmd.entity(entity).remove::<Flipping>();
        }
        let rotation_quaternion = Quat::from_rotation_y(rotation_angle.to_radians());
        transform.rotate(rotation_quaternion);
    }
}
