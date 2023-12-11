use std::time::Duration;

use bevy::{ecs::system::Command, prelude::*};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use self::{
    card::CardPlugin,
    deck::DeckPlugin,
    goals::{Goals, GoalsPlugin},
    hand::HandPlugin,
    rules::RulePlugin,
};
use super::ui::StartText;
use crate::{board::IsOnBoard, game_shapes::Shape, AppState};

pub mod card;
pub mod deck;
pub mod goals;
pub mod hand;
pub mod rules;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
pub enum GameState {
    #[default]
    Setup,
    Start,
    Draw,
    Playing,
    Discard,
    Scoring,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Actions {
    Select,
    Play,
}
#[derive(Resource)]
pub struct GameTimer {
    pub timer: Timer,
}
#[derive(Resource)]
pub struct Score {
    pub score: u32,
    pub base_score: u32,
    pub goal_status: Vec<bool>,
    pub cards_played: u32,
}
impl Score {
    pub fn reset(&mut self) {
        self.score = 0;
        self.base_score = 0;
        self.goal_status = vec![false, false, false];
        self.cards_played = 0;
    }
}

pub struct CardsPlugin;
impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .insert_resource(Score {
                score: 0,
                base_score: 0,
                goal_status: vec![false, false, false],
                cards_played: 0,
            })
            .insert_resource(GameTimer {
                timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
            })
            .add_plugins((DeckPlugin, HandPlugin, CardPlugin, RulePlugin, GoalsPlugin))
            .add_systems(OnEnter(AppState::Playing), setup_input)
            .add_systems(Update, start_game.run_if(in_state(GameState::Start)))
            .add_systems(OnExit(AppState::Playing), reset_resources)
            .add_systems(
                Update,
                time_game
                    .run_if(not(in_state(GameState::Setup)))
                    .run_if(not(in_state(GameState::Start))),
            )
            .add_plugins(InputManagerPlugin::<Actions>::default());
    }
}
pub fn reset_resources(
    mut cmd: Commands,
    mut game_timer: ResMut<GameTimer>,
    mut score: ResMut<Score>,
    mut q_actions: Query<Entity, With<ActionState<Actions>>>,
) {
    cmd.insert_resource(NextState(Some(GameState::Setup)));
    cmd.entity(q_actions.single()).despawn_recursive();

    game_timer.timer.reset();
    score.reset();
}
pub fn setup_input(mut cmd: Commands) {
    let mut input_map = InputMap::new([(MouseButton::Left, Actions::Select)]);
    input_map.insert(KeyCode::Space, Actions::Play);

    cmd.spawn((InputManagerBundle::<Actions> {
        action_state: ActionState::default(),
        input_map,
    },));
}
pub fn start_game(
    mut cmd: Commands,
    mut actions: Query<&ActionState<Actions>>,
    mut q_start_text: Query<&mut Visibility, With<StartText>>,
) {
    let action_state = actions.single();
    if action_state.just_pressed(Actions::Play) {
        let mut v = q_start_text.single_mut();
        *v = Visibility::Hidden;
        cmd.insert_resource(NextState(Some(GameState::Draw)));
    }
}
pub fn time_game(mut cmd: Commands, time: Res<Time>, mut game_timer: ResMut<GameTimer>) {
    game_timer.timer.tick(time.delta());
    if game_timer.timer.finished() {
        cmd.insert_resource(NextState(Some(GameState::Scoring)));
    }
}
