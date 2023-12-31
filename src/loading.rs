use bevy::{prelude::*, render::texture::ImageSampler};
use bevy_asset_loader::prelude::*;

use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Menu),
        )
        // .add_systems(Update, set_texture_tiled)
        // .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        // .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, SoundAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, FontAssets>(AppState::Loading);
    }
}

// pub fn set_texture_tiled(
//     mut texture_events: EventReader<AssetEvent<Image>>,
//     mut textures: ResMut<Assets<Image>>,
// ) {
//     for event in texture_events.read() {
//         match event {
//             AssetEvent::Added { id } => {
//                 if let Some(texture) = textures.get_mut(*id) {
//                     texture.sampler =
//                         ImageSampler::Descriptor(bevy::render::texture::ImageSamplerDescriptor {
//                             address_mode_u: bevy::render::texture::ImageAddressMode::Repeat,
//                             address_mode_v: bevy::render::texture::ImageAddressMode::Repeat,
//                             ..Default::default()
//                         })
//                 }
//             },
//             _ => (),
//         }
//     }
// }

// #[derive(AssetCollection, Resource)]
// pub struct FontAssets {
//     #[asset(path = "fonts/FiraSans-Bold.ttf")]
//     pub fira_sans: Handle<Font>,
// }

// #[derive(AssetCollection, Resource)]
// pub struct AudioAssets {
//     #[asset(path = "audio/flying.ogg")]
//     pub flying: Handle<AudioSource>,
// }

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "faces/card_red.png")]
    pub card_red: Handle<Image>,
    #[asset(path = "faces/card_blue.png")]
    pub card_blue: Handle<Image>,
    #[asset(path = "faces/blank.png")]
    pub card_blank: Handle<Image>,

    #[asset(path = "symbols/mul.png")]
    pub mul: Handle<Image>,
    #[asset(path = "symbols/sub.png")]
    pub sub: Handle<Image>,
    #[asset(path = "symbols/add.png")]
    pub add: Handle<Image>,
    #[asset(path = "symbols/two.png")]
    pub two: Handle<Image>,
    #[asset(path = "symbols/gt.png")]
    pub gt: Handle<Image>,
    #[asset(path = "background.png")]
    pub bg: Handle<Image>,
}
#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira: Handle<Font>,
}
#[derive(AssetCollection, Resource)]
pub struct SoundAssets {
    #[asset(path = "sounds/draw_card.ogg")]
    pub draw_card: Handle<AudioSource>,
    #[asset(path = "sounds/spawn_deck.ogg")]
    pub spawn_deck: Handle<AudioSource>,
    #[asset(path = "sounds/space_jazz.ogg")]
    pub bg_music: Handle<AudioSource>,
    #[asset(path = "sounds/pop.ogg")]
    pub spawn: Handle<AudioSource>,
}
