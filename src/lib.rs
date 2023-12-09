mod board;
mod camera;
mod cards;
mod game_shapes;
mod loading;
mod operation;
mod utils;

use std::default;

use bevy::prelude::*;
use board::BoardPlugin;
use camera::CameraPlugin;
use cards::CardsPlugin;
use loading::LoadingPlugin;

pub struct GamePlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugins(CameraPlugin)
            .add_plugins(CardsPlugin)
            .add_plugins(LoadingPlugin)
            .add_plugins(BoardPlugin);
    }
}
