use core::ops::{Add, Sub};

use bevy::{
    ecs::system::Command,
    prelude::{shape::RegularPolygon, *},
    sprite::MaterialMesh2dBundle,
    utils::HashMap,
};
use bevy_xpbd_2d::{self, prelude::Collider};
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{loading::TextureAssets, utils, AppState};

pub mod config {
    pub const POLYGON_RADIUS: f32 = 80.;
}

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameColor {
    Red,
    Green,
    Blue,
}
impl GameColor {
    pub fn random_color() -> GameColor {
        let mut rng = rand::thread_rng();
        GameColor::iter().choose(&mut rng).unwrap()
    }
    pub fn fight(self, other: GameColor) -> GameColor {
        match (self, other) {
            (GameColor::Red, GameColor::Red) => GameColor::Red,
            (GameColor::Red, GameColor::Green) => GameColor::Green,
            (GameColor::Red, GameColor::Blue) => GameColor::Red,
            (GameColor::Green, GameColor::Red) => GameColor::Green,
            (GameColor::Green, GameColor::Green) => GameColor::Green,
            (GameColor::Green, GameColor::Blue) => GameColor::Blue,
            (GameColor::Blue, GameColor::Red) => GameColor::Red,
            (GameColor::Blue, GameColor::Green) => GameColor::Blue,
            (GameColor::Blue, GameColor::Blue) => GameColor::Blue,
        }
    }
}
impl Into<Color> for GameColor {
    fn into(self) -> Color {
        match self {
            GameColor::Red => Color::RED,
            GameColor::Blue => Color::BLUE,
            GameColor::Green => Color::GREEN,
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
#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GamePolygon {
    Triangle,
    Square,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
}

const STARTING_SHAPE: [GamePolygon; 3] = [
    GamePolygon::Triangle,
    GamePolygon::Square,
    GamePolygon::Pentagon,
];

impl GamePolygon {
    fn get_vertices(self) -> u8 {
        match self {
            GamePolygon::Triangle => 3,
            GamePolygon::Square => 4,
            GamePolygon::Pentagon => 5,
            GamePolygon::Hexagon => 6,
            GamePolygon::Heptagon => 7,
            GamePolygon::Octagon => 8,
        }
    }
    fn from_vertices(n: usize) -> Option<GamePolygon> {
        match n {
            3 => Some(GamePolygon::Triangle),
            4 => Some(GamePolygon::Square),
            5 => Some(GamePolygon::Pentagon),
            6 => Some(GamePolygon::Hexagon),
            7 => Some(GamePolygon::Heptagon),
            8 => Some(GamePolygon::Octagon),
            _ => None,
        }
    }
    pub fn create_collider(self) -> Collider {
        // Collider::ball(config::POLYGON_RADIUS)
        Collider::convex_decomposition(
            utils::regular_polygon_vertices(self.get_vertices() as usize, config::POLYGON_RADIUS),
            (0..(self.get_vertices() as usize))
                .map(|i| [i as u32, ((i + 1) % (self.get_vertices() as usize)) as u32])
                .collect(),
        )
    }
    pub fn random_polygon() -> GamePolygon {
        let mut rng = rand::thread_rng();
        STARTING_SHAPE.iter().choose(&mut rng).unwrap().clone()
    }
}
impl Into<RegularPolygon> for GamePolygon {
    fn into(self) -> RegularPolygon {
        RegularPolygon {
            radius: config::POLYGON_RADIUS,
            sides: self.get_vertices().into(),
        }
    }
}
impl Add<Self> for GamePolygon {
    type Output = GamePolygon;

    fn add(self, rhs: Self) -> Self::Output {
        GamePolygon::from_vertices(self.get_vertices() as usize + rhs.get_vertices() as usize)
            .unwrap_or(GamePolygon::Octagon)
    }
}

impl Sub<Self> for GamePolygon {
    type Output = GamePolygon;

    fn sub(self, rhs: Self) -> Self::Output {
        GamePolygon::from_vertices(
            ((self.get_vertices() as i8) - (rhs.get_vertices() as i8)).max(3) as usize,
        )
        .unwrap_or(GamePolygon::Triangle)
    }
}

#[derive(Clone, Copy, Component, PartialEq, Eq, Debug)]
pub struct Shape {
    pub polygon: GamePolygon,
    pub color: GameColor,
}
impl Shape {
    pub fn get_bundle(
        self,
        ma: &Res<ShapeAssets>,
        c_m: &Res<ColorMaterialAssets>,
    ) -> ColorMesh2dBundle {
        ColorMesh2dBundle {
            mesh: get_polygon_mesh(&self.polygon, ma).into(),
            material: get_color_material(&self.color, c_m),
            ..Default::default()
        }
    }

    pub fn random_shape() -> Shape {
        Shape {
            polygon: GamePolygon::random_polygon(),
            color: GameColor::random_color(),
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct PolygonColliders(HashMap<GamePolygon, Collider>);

#[derive(Resource, Default)]
pub struct ShapeAssets {
    pub triangle: Handle<Mesh>,
    pub square: Handle<Mesh>,
    pub pentagon: Handle<Mesh>,
    pub hexagon: Handle<Mesh>,
    pub heptagon: Handle<Mesh>,
    pub octagon: Handle<Mesh>,
}

#[derive(Resource, Default)]
pub struct ColorMaterialAssets {
    pub red: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
}

pub fn get_polygon_mesh(p: &GamePolygon, ma: &Res<ShapeAssets>) -> Handle<Mesh> {
    match *p {
        GamePolygon::Triangle => ma.triangle.clone_weak(),
        GamePolygon::Square => ma.square.clone_weak(),
        GamePolygon::Pentagon => ma.pentagon.clone_weak(),
        GamePolygon::Hexagon => ma.hexagon.clone_weak(),
        GamePolygon::Heptagon => ma.heptagon.clone_weak(),
        GamePolygon::Octagon => ma.octagon.clone_weak(),
    }
}

pub fn get_color_material(p: &GameColor, c_m: &Res<ColorMaterialAssets>) -> Handle<ColorMaterial> {
    match *p {
        GameColor::Red => c_m.red.clone_weak(),
        GameColor::Green => c_m.green.clone_weak(),
        GameColor::Blue => c_m.blue.clone_weak(),
    }
}

pub struct GameShapePlugin;

impl Plugin for GameShapePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShapeAssets::default())
            .insert_resource(ColorMaterialAssets::default())
            .insert_resource(PolygonColliders(HashMap::new()))
            .add_systems(
                Startup,
                |mut a: ResMut<Assets<Mesh>>,
                 mut m: ResMut<Assets<ColorMaterial>>,
                 mut s_a: ResMut<ShapeAssets>,
                 mut p_c: ResMut<PolygonColliders>,
                 mut c_m_a: ResMut<ColorMaterialAssets>| {
                    s_a.triangle =
                        a.add(shape::RegularPolygon::new(config::POLYGON_RADIUS, 3).into());
                    s_a.square =
                        a.add(shape::RegularPolygon::new(config::POLYGON_RADIUS, 4).into());
                    s_a.pentagon =
                        a.add(shape::RegularPolygon::new(config::POLYGON_RADIUS, 5).into());
                    s_a.hexagon =
                        a.add(shape::RegularPolygon::new(config::POLYGON_RADIUS, 6).into());
                    s_a.heptagon =
                        a.add(shape::RegularPolygon::new(config::POLYGON_RADIUS, 7).into());
                    s_a.octagon =
                        a.add(shape::RegularPolygon::new(config::POLYGON_RADIUS, 8).into());

                    c_m_a.red = m.add(ColorMaterial::from(Color::RED));
                    c_m_a.green = m.add(ColorMaterial::from(Color::GREEN));
                    c_m_a.blue = m.add(ColorMaterial::from(Color::BLUE));

                    p_c.insert(
                        GamePolygon::Triangle,
                        GamePolygon::Triangle.create_collider(),
                    );

                    p_c.insert(GamePolygon::Square, GamePolygon::Square.create_collider());
                    p_c.insert(
                        GamePolygon::Pentagon,
                        GamePolygon::Pentagon.create_collider(),
                    );
                    p_c.insert(GamePolygon::Hexagon, GamePolygon::Hexagon.create_collider());
                    p_c.insert(
                        GamePolygon::Heptagon,
                        GamePolygon::Heptagon.create_collider(),
                    );
                    p_c.insert(GamePolygon::Octagon, GamePolygon::Octagon.create_collider());
                },
            );
    }
}
