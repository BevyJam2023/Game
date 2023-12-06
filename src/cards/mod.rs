use bevy::{ecs::system::Command, prelude::*};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use self::{card::CardPlugin, deck::DeckPlugin, hand::HandPlugin};
use crate::AppState;

mod card;
mod deck;
mod hand;
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
        app.add_plugins((DeckPlugin, HandPlugin, CardPlugin))
            .add_systems(OnEnter(AppState::Playing), setup_input)
            .add_plugins(InputManagerPlugin::<CardAction>::default());
    }
}
pub fn setup_input(mut cmd: Commands) {
    let mut input_map = InputMap::new([
        (MouseButton::Left, CardAction::Select),
        (MouseButton::Right, CardAction::Flip),
        (MouseButton::Middle, CardAction::Play),
    ]);
    input_map.insert(KeyCode::Space, CardAction::Draw);

    cmd.spawn((InputManagerBundle::<CardAction> {
        action_state: ActionState::default(),
        input_map,
    },));
}
