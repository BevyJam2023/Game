use std::ops::{Deref, Sub};

use bevy::{ecs::system::Command, prelude::*, render::texture::ImageSampler};
use bevy_xpbd_2d::prelude::{
    Collider, CollidingEntities, Collision, CollisionLayers, ExternalAngularImpulse, ExternalForce,
    ExternalImpulse, LinearVelocity, MassPropertiesBundle, PhysicsLayer, Restitution, RigidBody,
    SpatialQuery, SpatialQueryFilter,
};
use rand::{seq::IteratorRandom, Rng};

use crate::{
    cards::{self, deck::reset_deck, rules::Rule, GameState},
    game_shapes::{
        self, config::POLYGON_RADIUS, ColorMaterialAssets, GameColor, GamePolygon,
        PolygonColliders, Shape, ShapeAssets,
    },
    loading::{SoundAssets, TextureAssets},
    operation::Operation,
    utils::{average, vec3_to_vec2},
    AppState,
};

pub mod config {
    use super::Vec2;

    pub const SIZE: Vec2 = Vec2::new(1000., 1000.);
    pub const CENTER: Vec2 = Vec2::new(0., 0.2 * SIZE.x);
    pub const WALL_THICKNESS: f32 = 100.;
    pub const SHAPE_SCALE: f32 = 0.25;
    pub const MAX_SPEED: f32 = 100.;
    pub const MAX_SHAPES: u32 = 20_000;
    pub const MAX_RADIUS: f32 = 1_000.;
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
            .insert_resource(BoardTick(Timer::from_seconds(1.25, TimerMode::Repeating)))
            .add_systems(OnEnter(AppState::Playing), setup)
            .add_systems(OnExit(AppState::Playing), despawn_shapes)
            .add_systems(
                Update,
                (
                    spawn_bodies,
                    spawn_on_timer,
                    shape_collisions,
                    handle_delay,
                    clamp_vel,
                    world_gravity,
                )
                    .run_if(in_state(AppState::Playing))
                    .run_if(not(in_state(GameState::Scoring))),
            );
    }
}

fn setup(mut cmd: Commands, textures: Res<TextureAssets>) {
    cmd.spawn((
        SpriteBundle {
            texture: textures.bg.clone(),
            transform: Transform::from_translation(config::CENTER.extend(-10.))
                .with_scale(Vec3::splat(9999.)),
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

        // cmd.spawn((
        //     RigidBody::Static,
        //     Collider::cuboid(size.x, size.y),
        //     Restitution::PERFECTLY_ELASTIC,
        //     SpriteBundle {
        //         sprite: Sprite {
        //             color: Color::WHITE,
        //             custom_size: Some(Vec2::new(size.x, size.y)),
        //             ..Default::default()
        //         },
        //         transform: Transform::from_translation(position),
        //         ..Default::default()
        //     },
        //     CollisionLayers::new([Layer::Wall], [Layer::Wall]),
        // ));
    }
}

fn spawn_on_timer(
    mut cmd: Commands,
    mut board_tick: ResMut<BoardTick>,
    t: Res<Time>,
    rules: Query<&Rule>,
    q_board_shapes: Query<(Entity, &Shape), With<IsOnBoard>>,
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
                    * q_board_shapes.iter().filter(|(_, s)| s == &shape).count()))
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
                Operation::Dec(shape) => {
                    if let Some((e, _)) = q_board_shapes
                        .iter()
                        .filter(|(_, s)| s == &shape)
                        .choose(&mut rng_thread)
                    {
                        cmd.entity(e).despawn_recursive();
                    }
                    vec![]
                },
                _ => unreachable!(),
            })
            .flatten()
            .collect();

        e.send_batch(spawn_event);
        // for ev in spawn_event {
        //     e.send(ev);
        // }
        // e.send_batch(spawn_event);
    }

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
    q_board: Query<(), With<IsOnBoard>>,
    poly_colliders: Res<PolygonColliders>,
    mesh: Res<ShapeAssets>,
    color_mat: Res<ColorMaterialAssets>,
    r_sound: Res<SoundAssets>,
) {
    let frame_num = q_board.iter().collect::<Vec<()>>().len();

    let mut rng_thread = rand::thread_rng();

    for (i, event) in reader.read().enumerate() {
        if (i + frame_num) as u32 >= config::MAX_SHAPES {
            println!("Max reached");
            return;
        };

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
            ExternalForce::ZERO,
        ))
        .insert(event.transform.with_scale(Vec3::splat(config::SHAPE_SCALE)));

        cmd.spawn(AudioBundle {
            source: r_sound.spawn.clone_weak(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Remove,
                ..default()
            },
        });
    }
}

fn despawn_shapes(mut c: Commands, q_board_shapes: Query<(Entity, &Shape), With<IsOnBoard>>) {
    for (e, _) in q_board_shapes.iter() {
        c.entity(e).despawn_recursive();
    }
}

fn world_gravity(mut q_central_force: Query<(&mut ExternalForce, &Transform), With<IsOnBoard>>) {
    for (mut f, t) in q_central_force.iter_mut() {
        let v_to_center = config::CENTER - t.translation.truncate();

        let distance_to_center = t.translation.truncate().distance_squared(config::CENTER);

        f.set_force(distance_to_center * v_to_center.normalize_or_zero());
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
    mut cmd: Commands,
    rules: Query<&Rule>,
    q_shape: Query<(Entity, &Shape, &Transform, &LinearVelocity), With<IsOnBoard>>,
    // mut collision_event_reader: EventReader<Collision>,
    mut s_event: EventWriter<SpawnBody>,
) {
    let Ok(rule_ops) = rules.get_single() else {
        return;
    };

    let mut combined: Vec<&Entity> = Vec::new();

    let translations: Vec<(Entity, &Shape, Vec3, Vec2)> = q_shape
        .iter()
        .map(|(e, s, t, v)| (e, s, t.translation, v.0))
        .collect();

    for (ent, s, t, v) in translations.iter() {
        if combined.contains(&&ent) {
            continue;
        };

        if let Some((o_ent, o_s, o_t, o_v)) = translations
            .iter()
            .filter(|(o_ent, _, t, _)| {
                !combined.contains(&o_ent)
                    && (ent != o_ent)
                    && (t.distance_squared(*t)
                        < (1.05 * config::SHAPE_SCALE * POLYGON_RADIUS).powi(2))
            })
            .take(1)
            .collect::<Vec<&(Entity, &Shape, Vec3, Vec2)>>()
            .first()
        {
            let shapes_slc = [*s, *o_s];

            if let Some(spawn_event) = rule_ops
                .iter()
                .filter(|op| match op {
                    Operation::Add(s1, s2) => shapes_slc.contains(&s1) && shapes_slc.contains(&s2),
                    Operation::Sub(s1, s2) => shapes_slc.contains(&s1) && shapes_slc.contains(&s2),
                    _ => false,
                })
                .take(1)
                .collect::<Vec<&Operation>>()
                .first()
                .map(|op| match op {
                    Operation::Add(s1, s2) => SpawnBody {
                        shape: Shape {
                            polygon: s1.polygon + s2.polygon,
                            color: s1.color.fight(s2.color),
                        },
                        transform: Transform::from_translation(average(&[*t, *o_t])),
                        velocity: Some(LinearVelocity(average(&[*v, *o_v]))),
                    },
                    Operation::Sub(s1, s2) => SpawnBody {
                        shape: Shape {
                            polygon: s1.polygon - s2.polygon,
                            color: s1.color.fight(s2.color),
                        },
                        transform: Transform::from_translation(average(&[*t, *o_t])),
                        velocity: Some(LinearVelocity(average(&[*v, *o_v]))),
                    },
                    _ => unreachable!(),
                })
            {
                dbg!("combined", s, o_s, "into shape", spawn_event.shape);
                s_event.send(spawn_event);
                combined.append(&mut vec![ent, o_ent]);

                cmd.entity(*ent).despawn_recursive();
                cmd.entity(*o_ent).despawn_recursive();
            }
        }
    }
    //
    // // dbg!(rule_ops.deref());
    //
    //
    // for Collision(contacts) in collision_event_reader.read() {
    //     // TODO:
    //     // Combinations / interactions occur based on the 'Rules'
    //     //
    //     if combined.contains(&&contacts.entity1) || combined.contains(&&contacts.entity2) {
    //         continue;
    //     };
    //
    //     let polygons_slc = [c_s1.polygon, c_s2.polygon];
    //
    //     if let Some(spawn_event) = rule_ops
    //         .iter()
    //         .filter(|op| match op {
    //             Operation::Add(s1, s2) => {
    //                 polygons_slc.contains(&s1.polygon) && polygons_slc.contains(&s2.polygon)
    //             },
    //             Operation::Sub(s1, s2) => {
    //                 polygons_slc.contains(&s1.polygon) && polygons_slc.contains(&s2.polygon)
    //             },
    //             _ => false,
    //         })
    //         .last()
    //         .map(|op| match op {
    //             Operation::Add(s1, s2) => SpawnBody {
    //                 shape: Shape {
    //                     polygon: s1.polygon + s2.polygon,
    //                     color: s1.color.fight(s2.color),
    //                 },
    //                 transform: Transform::from_translation(average(&[
    //                     transform1.translation,
    //                     transform2.translation,
    //                 ])),
    //                 velocity: Some(LinearVelocity(average(&[*lin_v1.deref(), *lin_v2.deref()]))),
    //             },
    //             Operation::Sub(s1, s2) => SpawnBody {
    //                 shape: Shape {
    //                     polygon: s1.polygon - s2.polygon,
    //                     color: s1.color.fight(s2.color),
    //                 },
    //                 transform: Transform::from_translation(average(&[
    //                     transform1.translation,
    //                     transform2.translation,
    //                 ])),
    //                 velocity: Some(LinearVelocity(average(&[*lin_v1.deref(), *lin_v2.deref()]))),
    //             },
    //             _ => unreachable!(),
    //         })
    //     {
    //         println!("big collision");
    //         s_event.send(spawn_event);
    //         combined.append(&mut vec![&contacts.entity1, &contacts.entity2]);
    //     }
    // }

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
