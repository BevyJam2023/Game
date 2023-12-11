mod hud;
mod main_menu;
mod score_ui;
use bevy::prelude::*;

use self::{hud::HUDPlugin, main_menu::MainMenuPlugin, score_ui::ScoreUIPlugin};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(HUDPlugin);
        app.add_plugins(ScoreUIPlugin);
    }
}
#[derive(Component)]
pub struct StartText;
