use bevy::{ecs::system::Command, prelude::*};
use bevy_xpbd_2d::prelude::{
    Collider, Collision, ExternalAngularImpulse, ExternalImpulse, Restitution, RigidBody,
};
use rand::Rng;

use crate::{
    game_shapes::{self, ColorMaterialAssets, GameColor, GamePolygon, Shape, ShapeAssets},
    loading::TextureAssets,
    AppState,
};

pub mod config {
    use super::Vec2;

    pub const SIZE: Vec2 = Vec2::new(1000., 1000.);
    pub const CENTER: Vec2 = Vec2::new(0., 200.);
    pub const WALL_THICKNESS: f32 = 10.;
}

#[derive(Event)]
pub struct SpawnBody {
    shape: Shape,
    transform: Transform,
}

#[derive(Component)]
pub struct Board;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBody>()
            .add_systems(OnEnter(AppState::Playing), setup)
            .add_systems(
                Update,
                (spawn_bodies, spawn_on_timer).run_if(in_state(AppState::Playing)),
            );
    }
}

fn setup(mut cmd: Commands, textures: Res<TextureAssets>) {
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(config::SIZE.x, config::SIZE.y)),
                ..Default::default()
            },
            texture: textures.bg.clone(),
            transform: Transform::from_translation(config::CENTER.extend(0.)),
            ..Default::default()
        },
        Board,
    ));

    let side_length = config::SIZE.x;

    // Wall positions (assuming the center of the square is at the origin)
    let positions = [
        Vec3::new(0.0, side_length / 2.0 + config::WALL_THICKNESS / 2.0, 0.0), // Top wall
        Vec3::new(0.0, -side_length / 2.0 - config::WALL_THICKNESS / 2.0, 0.0), // Bottom wall
        Vec3::new(side_length / 2.0 + config::WALL_THICKNESS / 2.0, 0.0, 0.0), // Right wall
        Vec3::new(-side_length / 2.0 - config::WALL_THICKNESS / 2.0, 0.0, 0.0), // Left wall
    ];

    // Wall sizes
    let sizes = [
        Vec3::new(side_length, config::WALL_THICKNESS, 1.0), // Horizontal walls
        Vec3::new(config::WALL_THICKNESS, side_length, 1.0), // Vertical walls
    ];

    for (i, &position) in positions.iter().enumerate() {
        let size = if i < 2 { sizes[0] } else { sizes[1] };
        let position = position + config::CENTER.extend(0.);

        cmd.spawn((
            RigidBody::Static,
            // Transform::from_translation(position),
            Collider::cuboid(size.x, size.y),
            Restitution::PERFECTLY_ELASTIC,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                transform: Transform::from_translation(position),
                ..Default::default()
            },
        ));
    }
}

fn spawn_on_timer(t: Res<Time>, mut e: EventWriter<SpawnBody>) {
    // let mut rng_thread = rand::thread_rng();
    //
    // e.send(SpawnBody {
    //     shape: game_shapes::Shape {
    //         polygon: GamePolygon::Hexagon,
    //         color: GameColor::Blue,
    //     },
    //     transform: Transform::from_xyz(rng_thread.gen_range(-300..=300) as f32, 0., 10.),
    // });
}

fn spawn_bodies(
    mut cmd: Commands,
    mut reader: EventReader<SpawnBody>,
    mesh: Res<ShapeAssets>,
    color_mat: Res<ColorMaterialAssets>,
) {
    for event in reader.read() {
        cmd.spawn((
            event.shape.get_bundle(&mesh, &color_mat),
            event.shape.polygon.create_collider(),
            RigidBody::Dynamic,
            // ExternalImpulse::new(99999. * Vec2::Y).with_persistence(true),
            ExternalAngularImpulse::new(999.).with_persistence(true),
            Restitution::PERFECTLY_ELASTIC,
        ))
        .insert(event.transform.with_scale(Vec3::splat(0.5)));
    }
}

fn shape_collisions(mut collision_event_reader: EventReader<Collision>) {
    for Collision(contacts) in collision_event_reader.read() {
        // TODO:
        // Combinations / interactions occur based on the 'Rules'
    }
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

pub fn get_deck_transform(board_size: Vec2, card_height: f32) -> Transform {
    Transform::from_xyz(-board_size.x / 2., -(board_size.y + card_height) / 2., 20.)
}

pub fn get_discard_transform(board_size: Vec2, card_height: f32) -> Transform {
    Transform::from_xyz(board_size.x / 2., -(board_size.y + card_height) / 2., 20.)
}
