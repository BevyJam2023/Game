use bevy::{
    ecs::system::Command,
    prelude::{shape::RegularPolygon, *},
    sprite::MaterialMesh2dBundle,
};
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{loading::TextureAssets, AppState};

pub mod config {
    pub const POLYGON_RADIUS: f32 = 100.;
}
#[derive(EnumIter, Clone)]
pub enum GameColor {
    Red,
    Green,
    Blue,
    Yellow,
}
impl GameColor {
    fn random_color() -> GameColor {
        let mut rng = rand::thread_rng();
        GameColor::iter().choose(&mut rng).unwrap()
    }
}
impl Into<Color> for GameColor {
    fn into(self) -> Color {
        match self {
            GameColor::Red => Color::RED,
            GameColor::Blue => Color::BLUE,
            GameColor::Green => Color::GREEN,
            GameColor::Yellow => Color::YELLOW,
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
#[derive(EnumIter, Clone)]
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
            sides: self.get_vertices().into(),
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
    pub fn random_polygon() -> GamePolygon {
        let mut rng = rand::thread_rng();
        GamePolygon::iter().choose(&mut rng).unwrap()
    }
}
#[derive(Clone)]
pub struct Shape {
    polygon: GamePolygon,
    color: GameColor,
}
impl Shape {
    pub fn random_shape() -> Shape {
        Shape {
            polygon: GamePolygon::random_polygon(),
            color: GameColor::random_color(),
        }
    }
}

pub struct GameShapePlugin;

impl Plugin for GameShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            |mut cmd: Commands, mut a: ResMut<Assets<Mesh>>, m: ResMut<Assets<ColorMaterial>>| {
                // TODO:
                // Store Handles
            },
        );
    }
}
