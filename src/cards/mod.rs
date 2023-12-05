use bevy::prelude::*;
use leafwing_input_manager::{prelude::InputManagerPlugin, Actionlike};

use self::{card::CardPlugin, deck::DeckPlugin, hand::HandPlugin};

mod card;
mod deck;
mod hand;
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CardAction {
    Select,
    Flip,
    Draw,
}

pub struct CardsPlugin;
impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DeckPlugin, HandPlugin, CardPlugin))
            .add_plugins(InputManagerPlugin::<CardAction>::default());
    }
}
