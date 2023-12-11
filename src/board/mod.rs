use std::ops::{Deref, Sub};

use bevy::{ecs::system::Command, prelude::*};
use bevy_xpbd_2d::prelude::{
    Collider, CollidingEntities, Collision, CollisionLayers, ExternalAngularImpulse,
    ExternalImpulse, LinearVelocity, MassPropertiesBundle, PhysicsLayer, Restitution, RigidBody,
    SpatialQuery, SpatialQueryFilter,
};
use rand::Rng;

use crate::{
    cards::{self, rules::Rule, GameState},
    game_shapes::{
        self, config::POLYGON_RADIUS, ColorMaterialAssets, GameColor, GamePolygon,
        PolygonColliders, Shape, ShapeAssets,
    },
    loading::TextureAssets,
    operation::Operation,
    utils::average,
    AppState,
};

pub mod config {
    use super::Vec2;

    pub const SIZE: Vec2 = Vec2::new(1000., 1000.);
    pub const CENTER: Vec2 = Vec2::new(0., 0.2 * SIZE.x);
    pub const WALL_THICKNESS: f32 = 100.;
    pub const SHAPE_SCALE: f32 = 0.25;
    pub const MAX_SPEED: f32 = 100.;
}

#[derive(PhysicsLayer)]
enum Layer {
    Shape,
    Wall,
}

#[derive(Event, Clone, Copy)]
pub struct SpawnBody {
    shape: Shape,
    transform: Transform,
    velocity: Option<LinearVelocity>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct BoardTick(Timer);

#[derive(Component)]
pub struct Board;

#[derive(Component, Clone, Copy)]
pub struct AwaitNoCollision(usize);

#[derive(Component)]
pub struct IsOnBoard;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBody>()
            .insert_resource(BoardTick(Timer::from_seconds(0.025, TimerMode::Repeating)))
            .add_systems(OnEnter(AppState::Playing), setup)
            .add_systems(
                Update,
                (
                    spawn_bodies,
                    spawn_on_timer,
                    shape_collisions,
                    handle_delay,
                    clamp_vel,
                )
                    .run_if(in_state(AppState::Playing))
                    .run_if(not(in_state(GameState::Scoring))),
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
            CollisionLayers::new([Layer::Wall], [Layer::Shape]),
        ));
    }
}

fn spawn_on_timer(
    mut board_tick: ResMut<BoardTick>,
    t: Res<Time>,
    rules: Query<&Rule>,
    q_board_shapes: Query<&Shape, With<IsOnBoard>>,
    mut e: EventWriter<SpawnBody>,
) {
    if board_tick.tick(t.delta()).finished() {
        let mut rng_thread = rand::thread_rng();

        let Ok(rule_ops) = rules.get_single() else {
            return;
        };

        let spawn_event: Vec<SpawnBody> = rule_ops
            .iter()
            .filter(|op| match op {
                Operation::Mul(_, _) => true,
                // Operation::Sqr(_) => true,
                Operation::Inc(_) => true,
                Operation::Dec(_) => true,
                _ => false,
            })
            .map(|op| match op {
                Operation::Mul(shape, x) => (0..((*x as usize - 1)
                    * q_board_shapes.iter().filter(|s| s == &shape).count()))
                    .into_iter()
                    .map(|_| SpawnBody {
                        shape: shape.clone(),
                        transform: Transform::from_translation(
                            config::CENTER.extend(0.)
                                + Vec3::new(
                                    rng_thread.gen_range(-300..=300) as f32,
                                    rng_thread.gen_range(-300..=300) as f32,
                                    10.,
                                ),
                        ),
                        velocity: None,
                    })
                    .collect(),
                // Operation::Sqr(shape) => std::iter::repeat(SpawnBody {
                //     shape: shape.clone(),
                //     transform: Transform::from_translation(
                //         config::CENTER.extend(0.)
                //             + Vec3::new(
                //                 rng_thread.gen_range(-300..=300) as f32,
                //                 rng_thread.gen_range(-300..=300) as f32,
                //                 10.,
                //             ),
                //     ),
                //     velocity: None,
                // })
                // .take({
                //     let num = q_board_shapes.iter().filter(|s| s == &shape).count();
                //
                //     num.pow(2) - num
                // })
                // .collect(),
                Operation::Inc(shape) => vec![SpawnBody {
                    shape: shape.clone(),
                    transform: Transform::from_translation(
                        config::CENTER.extend(0.)
                            + Vec3::new(
                                rng_thread.gen_range(-300..=300) as f32,
                                rng_thread.gen_range(-300..=300) as f32,
                                10.,
                            ),
                    ),
                    velocity: None,
                }],
                Operation::Dec(_) => todo!("Not done Decrement"),
                _ => unreachable!(),
            })
            .flatten()
            .collect();

        for ev in spawn_event {
            e.send(ev);
        }
        // e.send_batch(spawn_event);
    }

    // NOTE:
    // Here for spawning many entities quickly...
    // let mut rng_thread = rand::thread_rng();
    // for i in (0..100) {
    //     e.send(SpawnBody {
    //         shape: Shape::random_shape(),
    //         transform: Transform::from_translation(
    //             config::CENTER.extend(0.)
    //                 + Vec3::new(
    //                     rng_thread.gen_range(-300..=300) as f32,
    //                     rng_thread.gen_range(-300..=300) as f32,
    //                     10.,
    //                 ),
    //         ),
    //         velocity: None,
    //     });
    // }
}

fn spawn_bodies(
    mut cmd: Commands,
    mut reader: EventReader<SpawnBody>,
    poly_colliders: Res<PolygonColliders>,
    mesh: Res<ShapeAssets>,
    color_mat: Res<ColorMaterialAssets>,
) {
    let mut rng_thread = rand::thread_rng();

    for event in reader.read() {
        cmd.spawn((
            event.shape.get_bundle(&mesh, &color_mat),
            poly_colliders.get(&event.shape.polygon).unwrap().clone(),
            event.shape.clone(),
            RigidBody::Dynamic,
            event.velocity.unwrap_or(LinearVelocity(Vec2::new(
                rng_thread.gen_range(-config::MAX_SPEED..=config::MAX_SPEED),
                rng_thread.gen_range(-config::MAX_SPEED..=config::MAX_SPEED),
            ))),
            Restitution::PERFECTLY_ELASTIC,
            IsOnBoard,
            CollisionLayers::new([Layer::Shape], [Layer::Wall]),
            AwaitNoCollision(300),
        ))
        .insert(event.transform.with_scale(Vec3::splat(config::SHAPE_SCALE)));
    }
}

fn clamp_vel(mut q_vel: Query<&mut LinearVelocity, With<IsOnBoard>>) {
    for mut v in q_vel
        .iter_mut()
        .filter(|v| v.0.dot(v.0) > config::MAX_SPEED.powi(2))
    {
        v.0 = v.length().min(config::MAX_SPEED) * v.normalize();
    }
}

fn handle_delay(
    mut c: Commands,
    mut q_coll: Query<(Entity, &mut AwaitNoCollision)>,
    q_trans: Query<(Entity, &Transform), With<IsOnBoard>>,
) {
    let translations: Vec<(Entity, Vec3)> =
        q_trans.iter().map(|(e, t)| (e, t.translation)).collect();

    for (ent, mut awa) in q_coll.iter_mut() {
        awa.0 = usize::max(1, awa.0 - 1);

        let (_, this_trans) = q_trans.get(ent).unwrap();

        let intersected = translations.iter().any(|(o_ent, t)| {
            (ent != *o_ent)
                && (this_trans.translation.distance_squared(*t)
                    < (2. * config::SHAPE_SCALE * POLYGON_RADIUS).powi(2))
        });

        if intersected && awa.0 == 1 {
            c.entity(ent).remove::<AwaitNoCollision>();
            c.entity(ent).insert(CollisionLayers::new(
                [Layer::Shape],
                [Layer::Shape, Layer::Wall],
            ));
        } else if intersected {
            // println!("overlapped");
        }
    }
}

fn shape_collisions(
    rules: Query<&Rule>,
    q_shape: Query<(&Shape, &Transform, &LinearVelocity)>,
    mut collision_event_reader: EventReader<Collision>,
    mut s_event: EventWriter<SpawnBody>,
) {
    let Ok(rule_ops) = rules.get_single() else {
        return;
    };

    // dbg!(rule_ops.deref());

    let mut combined: Vec<&Entity> = Vec::new();

    for Collision(contacts) in collision_event_reader.read() {
        // TODO:
        // Combinations / interactions occur based on the 'Rules'
        //
        let Ok((c_s1, transform1, lin_v1)) = q_shape.get(contacts.entity1) else {
            continue;
        };
        let Ok((c_s2, transform2, lin_v2)) = q_shape.get(contacts.entity2) else {
            continue;
        };
        if combined.contains(&&contacts.entity1) || combined.contains(&&contacts.entity2) {
            continue;
        };

        let polygons_slc = [c_s1.polygon, c_s2.polygon];

        if let Some(spawn_event) = rule_ops
            .iter()
            .filter(|op| match op {
                Operation::Add(s1, s2) => {
                    polygons_slc.contains(&s1.polygon) && polygons_slc.contains(&s2.polygon)
                },
                Operation::Sub(s1, s2) => {
                    polygons_slc.contains(&s1.polygon) && polygons_slc.contains(&s2.polygon)
                },
                _ => false,
            })
            .last()
            .map(|op| match op {
                Operation::Add(s1, s2) => SpawnBody {
                    shape: Shape {
                        polygon: s1.polygon + s2.polygon,
                        color: s1.color.fight(s2.color),
                    },
                    transform: Transform::from_translation(average(&[
                        transform1.translation,
                        transform2.translation,
                    ])),
                    velocity: Some(LinearVelocity(average(&[*lin_v1.deref(), *lin_v2.deref()]))),
                },
                Operation::Sub(s1, s2) => SpawnBody {
                    shape: Shape {
                        polygon: s1.polygon - s2.polygon,
                        color: s1.color.fight(s2.color),
                    },
                    transform: Transform::from_translation(average(&[
                        transform1.translation,
                        transform2.translation,
                    ])),
                    velocity: Some(LinearVelocity(average(&[*lin_v1.deref(), *lin_v2.deref()]))),
                },
                _ => unreachable!(),
            })
        {
            println!("big collision");
            s_event.send(spawn_event);
            combined.append(&mut vec![&contacts.entity1, &contacts.entity2]);
        }
    }

    // dbg!(combined.deref());
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
