use bevy::{ecs::system::Command, prelude::*};

use crate::{loading::TextureAssets, AppState};

pub mod config {
    pub const SIZE: (f32, f32) = (1000., 1000.);
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup);
    }
}

pub fn setup(mut cmd: Commands, textures: Res<TextureAssets>) {
    cmd.spawn(SpriteBundle {
        // transform: Transform::IDENTITY.with_scale(Vec3::splat(4.)),
        sprite: Sprite {
            custom_size: Some(Vec2::new(config::SIZE.0, config::SIZE.1)),
            ..Default::default()
        },
        texture: textures.card_king.clone(),
        ..Default::default()
    });
}
