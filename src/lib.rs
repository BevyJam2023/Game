mod board;
mod boids;
mod camera;
mod cards;
mod game_shapes;
mod loading;
mod operation;
mod ui;
mod utils;

use std::default;

use bevy::prelude::*;
use bevy_xpbd_2d::resources::Gravity;
use board::BoardPlugin;
use camera::CameraPlugin;
use cards::CardsPlugin;
use game_shapes::GameShapePlugin;
use loading::LoadingPlugin;
use ui::UIPlugin;

pub struct GamePlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    Instruction,
    // During this State the actual game logic is executed
    Playing,
    Menu,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .insert_resource(Gravity(Vec2::ZERO))
            .add_plugins((
                CameraPlugin,
                CardsPlugin,
                LoadingPlugin,
                GameShapePlugin,
                UIPlugin,
                BoardPlugin,
            ));
    }
}
