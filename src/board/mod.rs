use bevy::{ecs::system::Command, prelude::*};

use crate::{loading::TextureAssets, AppState};

pub mod config {
    pub const SIZE: (f32, f32) = (1000., 1000.);
}

#[derive(Component)]
pub struct Board;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup);
    }
}

fn setup(mut cmd: Commands, textures: Res<TextureAssets>) {
    cmd.spawn((
        SpriteBundle {
            transform: Transform::IDENTITY.with_translation(Vec3::new(0., 0., -100.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(config::SIZE.0, config::SIZE.1)),
                ..Default::default()
            },
            texture: textures.card_red.clone(),
            ..Default::default()
        },
        Board,
    ));
}

// NOTE:
// Reference for getting image size :
// if let Some(image) = images.get(&textures.card_king) {
//     // Get the dimensions of the image
//     let dimensions = image.texture_descriptor.size;
//     println!("Image Dimensions: {:?}", dimensions);
// } else {
//     println!("Image not loaded yet");
// }

pub fn get_deck_transform(board_size: Vec2, card_height: f32) {}
