mod main_menu;
use bevy::prelude::*;

use self::main_menu::MainMenuPlugin;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin);
    }
}
