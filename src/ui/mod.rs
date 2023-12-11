mod hud;
mod main_menu;
use bevy::prelude::*;

use self::{hud::HUDPlugin, main_menu::MainMenuPlugin};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(HUDPlugin);
    }
}
#[derive(Component)]
pub struct StartText;
