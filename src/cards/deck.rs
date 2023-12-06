use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_tweening::Lerp;
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use super::{
    card::{Card, CardBundle, CardFace, FlipCard},
    hand::Hand,
    CardAction,
};
use crate::{loading::TextureAssets, AppState};

#[derive(Component)]
pub struct Deck {
    pub size: usize,
}
#[derive(Component)]
pub struct Discard;
#[derive(Component)]
pub struct Library;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), (spawn_deck, spawn_discard))
            .add_event::<ShuffleDiscard>()
            .add_systems(
                Update,
                (position_cards, draw_card, discard_into_library)
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
fn spawn_discard(mut cmd: Commands) {
    cmd.spawn((
        Discard,
        Deck { size: 60 },
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(100., -300., 0.),
                ..default()
            },
            ..default()
        },
    ));
}

//spawn deck when deck plugin is made
fn spawn_deck(mut cmd: Commands, textures: Res<TextureAssets>) {
    let deck_id = cmd
        .spawn((
            Library,
            Deck { size: 60 },
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(-400., -300., 0.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    //TODO make this a make_card func
    for i in 0..60 {
        let front = cmd
            .spawn((
                SpriteBundle {
                    texture: textures.card_king.clone(),
                    visibility: Visibility::Hidden,
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
                    face_up: false,
                },
                sprite: SpriteBundle { ..default() },
            })
            .id();

        cmd.entity(card_id).push_children(&[front, back]);
        cmd.entity(deck_id).push_children(&[card_id]);
    }
}
fn position_cards(
    q_deck: Query<(&Transform, &Deck, &Children)>,
    mut q_cards: Query<(&Card, &mut Transform), Without<Deck>>,
) {
    for (deck_t, deck, children) in q_deck.iter() {
        for (i, &child) in children.iter().enumerate() {
            if let Ok((card, mut transform)) = q_cards.get_mut(child) {
                transform.translation.x = transform.translation.x.lerp(&0., &0.2);
                transform.translation.y = transform.translation.y.lerp(&(i as f32 * 0.5), &0.2);

                transform.translation.z = i as f32;
            }
        }
    }
}

pub fn draw_card(
    mut cmd: Commands,
    actions: Query<&ActionState<CardAction>>,
    mut query: Query<(&Transform, &mut Deck, &mut Children), (With<Library>, Without<Card>)>,
    mut q_cards: Query<(&Card, &mut Transform)>,
    mut hand: Query<(Entity, &mut Hand)>,
    mut flip_writer: EventWriter<FlipCard>,
    mut shuffle_discard_writer: EventWriter<ShuffleDiscard>,
) {
    if let Ok((deck_transform, mut deck, children)) = query.get_single_mut() {
        let action_state = actions.single();
        if action_state.just_pressed(CardAction::Draw) {
            let (entity, mut hand) = hand.single_mut();
            for &child in children.iter() {
                if let Ok((card, mut card_transform)) = q_cards.get_mut(child) {
                    cmd.entity(child).remove_parent();

                    card_transform.translation.x += deck_transform.translation.x;
                    card_transform.translation.y += deck_transform.translation.y;

                    deck.size -= 1;
                    cmd.entity(entity).push_children(&[child]);
                    hand.size += 1;
                    flip_writer.send(FlipCard { card: child });

                    return;
                }
            }
        }
    } else {
        shuffle_discard_writer.send(ShuffleDiscard);
    }
}
#[derive(Event)]
pub struct ShuffleDiscard;

pub fn discard_into_library(
    mut cmd: Commands,
    mut q_library: Query<(Entity, &Transform, &mut Deck), (With<Library>, Without<Discard>)>,
    mut q_discard: Query<(&Transform, &mut Deck, &mut Children), (With<Discard>, Without<Card>)>,
    mut q_cards: Query<(&Card, &mut Transform), Without<Library>>,
    mut event: EventReader<ShuffleDiscard>,
    mut flip_writer: EventWriter<FlipCard>,
) {
    for e in event.read() {
        let (library_e, library_t, mut library_d) = q_library.single_mut();
        let (discard_t, mut discard_d, children) = q_discard.single_mut();
        library_d.size += discard_d.size;

        discard_d.size = 0;

        for &child in children.iter() {
            if let Ok((card, mut card_t)) = q_cards.get_mut(child) {
                cmd.entity(child).remove_parent();
                card_t.translation.x += discard_t.translation.x - library_t.translation.x;
                card_t.translation.y += discard_t.translation.y - library_t.translation.y;
                cmd.entity(library_e).push_children(&[child]);
                flip_writer.send(FlipCard { card: child });
            }
        }
    }
}
