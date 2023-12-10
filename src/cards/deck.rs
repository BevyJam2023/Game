use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_tweening::Lerp;
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use super::{
    card::{Card, CardBundle, CardFace, FlipCard, Flipping, SpawnCard},
    hand::Hand,
    CardAction, GameState,
};
use crate::{
    board,
    loading::TextureAssets,
    operation::{generate_random_operations, Operation},
    AppState,
};

#[derive(Component)]
pub struct Deck;
#[derive(Component)]
pub struct Discard;
#[derive(Component)]
pub struct Library;

#[derive(Resource)]
pub struct DeckSetup {
    deck_setup_timer: Timer,
    draw_timer: Timer,
    discard_timer: Timer,
    spawned: usize,
    hand_size: usize,
    library_operations: Vec<Operation>,
}
#[derive(Event)]
pub struct DrawCard;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), (spawn_deck, spawn_discard))
            .add_event::<DrawCard>()
            .add_event::<ShuffleDiscard>()
            .add_systems(
                Update,
                (
                    (
                        (draw_card, discard_into_library),
                        setup_decks.run_if(in_state(GameState::Setup)),
                        draw_to_hand_size.run_if(in_state(GameState::Draw)),
                        discard_hand.run_if(in_state(GameState::Discard)),
                    )
                        .run_if(in_state(AppState::Playing)),
                    position_cards,
                ),
            )
            .insert_resource(DeckSetup {
                deck_setup_timer: Timer::from_seconds(0.01, TimerMode::Repeating),
                draw_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                discard_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                spawned: 0,
                hand_size: 5,
                library_operations: generate_random_operations(60),
            });
    }
}
fn setup_decks(
    mut cmd: Commands,
    time: Res<Time>,
    mut deck_setup: ResMut<DeckSetup>,
    mut writer: EventWriter<SpawnCard>,
    mut q_library: Query<Entity, (With<Library>, Without<Discard>)>,
    // mut q_discard: Query<(&Transform, &mut Deck, &mut Children), (With<Discard>, Without<Card>)>,
    mut game_state: ResMut<State<GameState>>,
) {
    let entity = q_library.single();
    deck_setup.deck_setup_timer.tick(time.delta());
    if deck_setup.deck_setup_timer.finished() {
        writer.send(SpawnCard {
            operation: deck_setup.library_operations[deck_setup.spawned].clone(),
            zone_id: entity,
            face_up: false,
        });
        deck_setup.spawned += 1;
    }
    if deck_setup.spawned >= deck_setup.library_operations.len() {
        deck_setup.deck_setup_timer.reset();
        deck_setup.spawned = 0;
        cmd.insert_resource(NextState(game_state.next_state()))
    }
}
fn discard_hand(
    mut cmd: Commands,
    time: Res<Time>,
    mut deck_setup: ResMut<DeckSetup>,
    mut q_hand: Query<(&Children, &Transform), (With<Hand>, Without<Card>)>,
    mut q_discard: Query<(Entity, &mut Transform), (With<Discard>, Without<Hand>)>,
    mut q_cards: Query<(Entity, &mut Transform), (With<Card>, Without<Discard>)>,
    mut flip_writer: EventWriter<FlipCard>,
) {
    if q_hand.is_empty() {
        deck_setup.discard_timer.reset();
        cmd.insert_resource(NextState(Some(GameState::Draw)));
        return;
    }

    deck_setup.discard_timer.tick(time.delta());

    if deck_setup.discard_timer.finished() {
        let (children, h_transform) = q_hand.single();
        let (discard_e, discard_t) = q_discard.single();
        let &child = children.first().unwrap();
        if let Ok((card, mut card_transform)) = q_cards.get_mut(child) {
            flip_writer.send(FlipCard { card: child });

            cmd.entity(child).remove_parent();

            card_transform.translation.x += h_transform.translation.x - discard_t.translation.x;
            card_transform.translation.y += h_transform.translation.y - discard_t.translation.y;

            cmd.entity(discard_e).insert_children(0, &[child]);
        }
    }
}
fn draw_to_hand_size(
    mut cmd: Commands,
    time: Res<Time>,
    mut deck_setup: ResMut<DeckSetup>,
    mut writer: EventWriter<DrawCard>,
    mut game_state: ResMut<State<GameState>>,
) {
    deck_setup.draw_timer.tick(time.delta());

    if deck_setup.draw_timer.finished() {
        writer.send(DrawCard);
        deck_setup.spawned += 1;
        deck_setup.draw_timer.reset();
    }
    if deck_setup.spawned >= deck_setup.hand_size {
        deck_setup.spawned = 0;

        cmd.insert_resource(NextState(game_state.next_state()))
    }
}

fn spawn_discard(mut cmd: Commands) {
    cmd.spawn((
        Discard,
        Deck,
        SpatialBundle {
            transform: board::get_discard_transform(board::config::SIZE.into(), 190. + 50.),
            ..default()
        },
    ));
}

//spawn deck when deck plugin is made
fn spawn_deck(mut cmd: Commands, textures: Res<TextureAssets>) {
    let deck_id = cmd
        .spawn((
            Library,
            Deck,
            SpatialBundle {
                transform: board::get_deck_transform(board::config::SIZE.into(), 190. + 50.), // Card
                // Image size plus the offset of the stack cards...
                ..default()
            },
        ))
        .id();
}
fn position_cards(
    q_deck: Query<(&Transform, &Deck, &Children)>,
    mut q_cards: Query<(&Card, &mut Transform), Without<Deck>>,
    mut q_flipping: Query<&Flipping>,
) {
    for (deck_t, deck, children) in q_deck.iter() {
        for (i, &child) in children.iter().enumerate() {
            if let Ok((card, mut transform)) = q_cards.get_mut(child) {
                transform.translation.x = transform.translation.x.lerp(&0., &0.2);
                transform.translation.y = transform
                    .translation
                    .y
                    .lerp(&((children.len() - 1 - i) as f32 * 0.5), &0.2);

                transform.translation.z = (children.len() - 1 - i) as f32;
                if !q_flipping.contains(child) {
                    let before = transform.rotation.to_euler(EulerRot::XYZ);
                    let mut rot: f32 = 0.;
                    if card.face_up {
                        rot += 180.;
                    }

                    transform.rotation = transform.rotation.lerp(
                        Quat::from_euler(EulerRot::XYZ, before.0, before.1, rot.to_radians()),
                        0.2,
                    );
                }
            }
        }
    }
}

pub fn draw_card(
    mut cmd: Commands,
    mut query: Query<(&Transform, &mut Deck, &mut Children), (With<Library>, Without<Card>)>,
    mut q_cards: Query<(&Card, &mut Transform), Without<Hand>>,
    mut hand: Query<(Entity, &mut Hand, &mut Transform), Without<Library>>,
    mut reader: EventReader<DrawCard>,
    mut flip_writer: EventWriter<FlipCard>,
    mut shuffle_discard_writer: EventWriter<ShuffleDiscard>,
) {
    for event in reader.read() {
        if let Ok((deck_transform, mut deck, children)) = query.get_single_mut() {
            let (entity, mut hand, mut hand_transform) = hand.single_mut();
            let &child = children.first().unwrap();
            if let Ok((card, mut card_transform)) = q_cards.get_mut(child) {
                cmd.entity(child).remove_parent();

                card_transform.translation.x +=
                    deck_transform.translation.x - hand_transform.translation.x;
                card_transform.translation.y +=
                    deck_transform.translation.y - hand_transform.translation.y;

                cmd.entity(entity).push_children(&[child]);
                flip_writer.send(FlipCard { card: child });
            }
        } else {
            shuffle_discard_writer.send(ShuffleDiscard);
        }
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

        for &child in children.iter() {
            if let Ok((card, mut card_t)) = q_cards.get_mut(child) {
                cmd.entity(child).remove_parent();
                card_t.translation.x += discard_t.translation.x - library_t.translation.x;
                card_t.translation.y += discard_t.translation.y - library_t.translation.y;
                cmd.entity(library_e).push_children(&[child]);
            }
        }
    }
}
