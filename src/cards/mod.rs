use bevy::{ecs::system::Command, prelude::*};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use self::{card::CardPlugin, deck::DeckPlugin, hand::HandPlugin, rules::RulePlugin};
use crate::AppState;

mod card;
mod criteria;
mod deck;
mod hand;
mod rules;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
pub enum GameState {
    #[default]
    Setup,
    Draw,
    Playing,
    Discard,
}
impl GameState {
    pub fn next_state(&self) -> Option<Self> {
        match self {
            Self::Setup => Some(Self::Draw),
            Self::Draw => Some(Self::Playing),
            Self::Playing => Some(Self::Discard),
            Self::Discard => Some(Self::Draw),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CardAction {
    Select,
    Flip,
    Draw,
    Play,
}

pub struct CardsPlugin;
impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((DeckPlugin, HandPlugin, CardPlugin, RulePlugin))
            .add_systems(OnEnter(AppState::Playing), setup_input)
            .add_plugins(InputManagerPlugin::<CardAction>::default());
    }
}
pub fn setup_input(mut cmd: Commands) {
    let mut input_map = InputMap::new([
        (MouseButton::Left, CardAction::Select),
        (MouseButton::Right, CardAction::Flip),
    ]);
    input_map.insert(KeyCode::Space, CardAction::Play);

    cmd.spawn((InputManagerBundle::<CardAction> {
        action_state: ActionState::default(),
        input_map,
    },));
}
