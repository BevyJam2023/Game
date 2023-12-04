use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Playing),
        )
        // .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        // .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(AppState::Loading);
    }
}

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
    #[asset(path = "faces/cardSpadesK.png")]
    pub card_king: Handle<Image>,
    #[asset(path = "faces/cardSpadesA.png")]
    pub card_ace: Handle<Image>,
}
