use bevy::{
    ecs::system::Command,
    prelude::{shape::RegularPolygon, *},
};

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
}

pub struct GameShape {
    polygon: GamePolygon,
    color: GameColor,
}
impl GameShape {
    fn get_bundle(self, ma: &Res<ShapeAssets>, c_m : &Res<ColorMaterialAssets>) -> ColorMesh2dBundle {
        ColorMesh2dBundle {
            mesh: get_polygon_mesh(&self.polygon, ma).into(),
            material: get_color_material(&self.color, c_m),
            ..Default::default()
        }
    }
}

#[derive(Resource, Default)]
pub struct ShapeAssets {
    pub triangle: Handle<Mesh>,
    pub square: Handle<Mesh>,
    pub pentagon: Handle<Mesh>,
    pub hexagon: Handle<Mesh>,
}

#[derive(Resource, Default)]
pub struct ColorMaterialAssets {
    pub red: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
    pub yellow: Handle<ColorMaterial>,
}

pub fn get_polygon_mesh(p: &GamePolygon, ma: &Res<ShapeAssets>) -> Handle<Mesh> {
    match *p {
        GamePolygon::Triangle => ma.triangle.clone_weak(),
        GamePolygon::Square => ma.square.clone_weak(),
        GamePolygon::Pentagon => ma.pentagon.clone_weak(),
        GamePolygon::Hexagon => ma.hexagon.clone_weak(),
    }
}

pub fn get_color_material(p: &GameColor, c_m : &Res<ColorMaterialAssets>) -> Handle<ColorMaterial> {
    match *p {
        GameColor::Red => c_m.red.clone_weak(),
        GameColor::Green => c_m.green.clone_weak(),
        GameColor::Blue => c_m.blue.clone_weak(),
        GameColor::Yellow => c_m.yellow.clone_weak(),
    }
}

pub struct GameShapePlugin;

impl Plugin for GameShapePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShapeAssets::default())
            .insert_resource(ColorMaterialAssets::default())
            .add_systems(
                Startup,
                |mut a: ResMut<Assets<Mesh>>,
                 mut m: ResMut<Assets<ColorMaterial>>,
                 mut s_a: ResMut<ShapeAssets>,
                 mut c_m_a: ResMut<ColorMaterialAssets>| {
                    s_a.triangle =
                        a.add(shape::RegularPolygon::new(config::POLYGON_RADIUS, 3).into());
                    s_a.square =
                        a.add(shape::RegularPolygon::new(config::POLYGON_RADIUS, 4).into());
                    s_a.pentagon =
                        a.add(shape::RegularPolygon::new(config::POLYGON_RADIUS, 5).into());
                    s_a.hexagon =
                        a.add(shape::RegularPolygon::new(config::POLYGON_RADIUS, 6).into());

                    c_m_a.red = m.add(ColorMaterial::from(RED));
                    c_m_a.green = m.add(ColorMaterial::from(GREEN));
                    c_m_a.blue = m.add(ColorMaterial::from(BLUE));
                    c_m_a.yellow = m.add(ColorMaterial::from(YELLOW));
                },
            );
    }
}
