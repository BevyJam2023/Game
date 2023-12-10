#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_tweening::*;
use bevy_xpbd_2d::prelude::{PhysicsDebugPlugin, PhysicsPlugins};
// use bevy_xpbd
use sham::GamePlugin;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            TweeningPlugin,
            GamePlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Whoot".to_string(),
                    resolution: (1920., 1080.).into(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }),
        ))
        .run();
}
