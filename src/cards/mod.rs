use std::time::Duration;

use bevy::{ecs::system::Command, prelude::*};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use self::{
    card::CardPlugin, deck::DeckPlugin, goals::GoalsPlugin, hand::HandPlugin, rules::RulePlugin,
};
use super::ui::StartText;
use crate::AppState;

mod card;
mod deck;
mod goals;
mod hand;
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

pub struct CardsPlugin;
impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .insert_resource(GameTimer {
                timer: Timer::new(Duration::from_secs(120), TimerMode::Once),
            })
            .add_plugins((DeckPlugin, HandPlugin, CardPlugin, RulePlugin, GoalsPlugin))
            .add_systems(OnEnter(AppState::Playing), setup_input)
            .add_systems(Update, start_game.run_if(in_state(GameState::Start)))
            .add_systems(
                Update,
                time_game
                    .run_if(not(in_state(GameState::Setup)))
                    .run_if(not(in_state(GameState::Start))),
            )
            .add_plugins(InputManagerPlugin::<Actions>::default());
    }
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
