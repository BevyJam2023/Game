use bevy::{ecs::system::Command, prelude::{*, shape::RegularPolygon}};

use crate::{loading::TextureAssets, AppState};

pub const RED: Color = Color::RED;
pub const GREEN: Color = Color::GREEN;
pub const BLUE: Color = Color::BLUE;
pub const YELLOW: Color = Color::YELLOW;

pub mod config {
    pub const POLYGON_RADIUS: f32 = 100.;
}

pub enum GameColor {
    Red,
    Green,
    Blue,
    Yellow,
}
impl Into<Color> for GameColor {
    fn into(self) -> Color {
        match self {
            GameColor::Red => RED,
            GameColor::Blue => BLUE,
            GameColor::Green => GREEN,
            GameColor::Yellow => YELLOW,
        }
    }
}
impl Into<ColorMaterial> for GameColor {
    fn into(self) -> ColorMaterial {
        ColorMaterial {
            color: self.into(),
            texture: None,
        }
    }
}

pub enum GamePolygon {
    Triangle,
    Square,
    Pentagon,
    Hexagon,
}
impl Into<RegularPolygon> for GamePolygon {
    fn into(self) -> RegularPolygon {
        RegularPolygon {
            radius: config::POLYGON_RADIUS,
            sides: self.get_vertices().into()
        }
    }
}
impl GamePolygon {
    fn get_vertices(self) -> u8 {
        match self {
            GamePolygon::Triangle => 3,
            GamePolygon::Square => 4,
            GamePolygon::Pentagon => 5,
            GamePolygon::Hexagon => 6,
        }
}
}

pub struct GameShape {
    polygon: GamePolygon,
    color: GameColor
}

pub struct 

pub struct GameShapePlugin;

impl Plugin for GameShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut cmd: Commands, mut a: ResMut<Assets<Mesh>>, m: ResMut<Assets<ColorMaterial>>| {
            // TODO:
            // Store Handles
        });
    }
}
