use std::iter::repeat_with;

use bevy::{prelude::*, render::view::RenderLayers, sprite::SpriteBundle, utils::default};
use rand::{seq::IteratorRandom, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    game_shapes::{ColorMaterialAssets, Shape, ShapeAssets},
    loading::TextureAssets,
};
#[derive(Clone, Debug)]
pub enum Operation {
    Mul(Shape, u32),
    Sub(Shape, Shape),
    Add(Shape, Shape),
    // Sqr(Shape),
    Inc(Shape),
    Dec(Shape),
    None,
}
impl Operation {
    // pub fn get_operation_asset(textures: ResMut<TextureAssets>) -> Handle<Image> {
    //     match  {
    //
    //     }
    // }
    pub fn random_operation() -> Operation {
        let mut rng = rand::thread_rng();

        let o = rng.gen_range(0..100);
        match o {
            0..=9 => Operation::Mul(Shape::random_shape(), 2),
            10..=19 => Operation::Sub(Shape::random_shape(), Shape::random_shape()),
            20..=39 => Operation::Add(Shape::random_shape(), Shape::random_shape()),
            40..=69 => Operation::Inc(Shape::random_shape()),
            70..=89 => Operation::Dec(Shape::random_shape()),
            _ => Operation::None,
        }
    }

    pub(crate) fn get_operation_entity(
        &self,
        cmd: &mut bevy::prelude::Commands<'_, '_>,
        textures: &Res<TextureAssets>,

        ma: &Res<ShapeAssets>,
        c_m: &Res<ColorMaterialAssets>,
    ) -> Vec<Entity> {
        match self {
            Operation::Mul(s, i) => {
                vec![
                    cmd.spawn(s.get_bundle(ma, c_m))
                        .insert(Transform {
                            translation: Vec3::new(-40., 0., 1.),
                            scale: Vec3::new(0.25, 0.25, 1.),
                            ..default()
                        })
                        .insert(RenderLayers::layer(1))
                        .id(),
                    cmd.spawn(SpriteBundle {
                        texture: textures.mul.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., 0., 1.),
                            scale: Vec3::new(0.4, 0.4, 1.),
                            ..default()
                        },

                        ..default()
                    })
                    .insert(RenderLayers::layer(1))
                    .id(),
                    cmd.spawn(SpriteBundle {
                        texture: textures.two.clone(),
                        transform: Transform {
                            translation: Vec3::new(40., 0., 1.),
                            scale: Vec3::new(0.9, 0.9, 1.),
                            ..default()
                        },

                        ..default()
                    })
                    .insert(RenderLayers::layer(1))
                    .id(),
                ]
            },
            Operation::Sub(s1, s2) => {
                vec![
                    cmd.spawn(s1.get_bundle(ma, c_m))
                        .insert(Transform {
                            translation: Vec3::new(-40., 0., 1.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        })
                        .insert(RenderLayers::layer(1))
                        .id(),
                    cmd.spawn(SpriteBundle {
                        texture: textures.sub.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., 0., 1.),
                            scale: Vec3::new(0.4, 0.4, 1.),
                            ..default()
                        },

                        ..default()
                    })
                    .insert(RenderLayers::layer(1))
                    .id(),
                    cmd.spawn(s2.get_bundle(ma, c_m))
                        .insert(Transform {
                            translation: Vec3::new(40., 0., 1.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        })
                        .insert(RenderLayers::layer(1))
                        .id(),
                ]
            },
            Operation::Add(s1, s2) => {
                vec![
                    cmd.spawn(s1.get_bundle(ma, c_m))
                        .insert(Transform {
                            translation: Vec3::new(-40., 0., 1.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        })
                        .insert(RenderLayers::layer(1))
                        .id(),
                    cmd.spawn(SpriteBundle {
                        texture: textures.add.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., 0., 1.),
                            scale: Vec3::new(0.4, 0.4, 1.),
                            ..default()
                        },

                        ..default()
                    })
                    .insert(RenderLayers::layer(1))
                    .id(),
                    cmd.spawn(s2.get_bundle(ma, c_m))
                        .insert(Transform {
                            translation: Vec3::new(40., 0., 1.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        })
                        .insert(RenderLayers::layer(1))
                        .id(),
                ]
            },
            // Operation::Sqr(s) => {
            //     vec![
            //         cmd.spawn(s.get_bundle(ma, c_m))
            //             .insert(Transform {
            //                 translation: Vec3::new(0., 0., 1.),
            //                 scale: Vec3::new(0.3, 0.3, 1.),
            //                 ..default()
            //             })
            //             .id(),
            //         cmd.spawn(SpriteBundle {
            //             texture: textures.two.clone(),
            //             transform: Transform {
            //                 translation: Vec3::new(40., 40., 1.),
            //                 scale: Vec3::new(0.5, 0.5, 1.),
            //                 ..default()
            //             },
            //
            //             ..default()
            //         })
            //         .id(),
            //     ]
            // },
            Operation::None => vec![],
            Operation::Inc(s) => {
                vec![
                    cmd.spawn(s.get_bundle(ma, c_m))
                        .insert(Transform {
                            translation: Vec3::new(0., 0., 1.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        })
                        .insert(RenderLayers::layer(1))
                        .id(),
                    cmd.spawn(SpriteBundle {
                        texture: textures.add.clone(),
                        transform: Transform {
                            translation: Vec3::new(40., 0., 1.),
                            scale: Vec3::new(0.5, 0.5, 1.),
                            ..default()
                        },

                        ..default()
                    })
                    .insert(RenderLayers::layer(1))
                    .id(),
                ]
            },
            Operation::Dec(s) => {
                vec![
                    cmd.spawn(s.get_bundle(ma, c_m))
                        .insert(Transform {
                            translation: Vec3::new(0., 0., 1.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        })
                        .insert(RenderLayers::layer(1))
                        .id(),
                    cmd.spawn(SpriteBundle {
                        texture: textures.sub.clone(),
                        transform: Transform {
                            translation: Vec3::new(40., 0., 1.),
                            scale: Vec3::new(0.5, 0.5, 1.),
                            ..default()
                        },

                        ..default()
                    })
                    .insert(RenderLayers::layer(1))
                    .id(),
                ]
            },
        }
    }
}
pub fn generate_random_operations(num: usize) -> Vec<Operation> {
    let mut rng = rand::thread_rng();

    repeat_with(|| Operation::random_operation())
        .take(num)
        .collect()
}
