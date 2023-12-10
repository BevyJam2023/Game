use std::iter::repeat_with;

use bevy::prelude::*;

use crate::{
    game_shapes::{ColorMaterialAssets, Shape, ShapeAssets},
    loading::TextureAssets,
    operation::Operation,
};
#[derive(Clone, Debug)]
pub struct Goal {
    s1: Shape,
    s2: Shape,
}
impl Goal {
    pub fn random_goal() -> Self {
        Goal {
            s1: Shape::random_shape(),
            s2: Shape::random_shape(),
        }
    }
    pub(crate) fn get_goal_entity(
        &self,
        cmd: &mut bevy::prelude::Commands<'_, '_>,
        textures: &Res<TextureAssets>,

        ma: &Res<ShapeAssets>,
        c_m: &Res<ColorMaterialAssets>,
    ) -> Vec<Entity> {
        vec![
            cmd.spawn(self.s1.get_bundle(ma, c_m))
                .insert(Transform {
                    translation: Vec3::new(-40., 0., 200.),
                    scale: Vec3::new(0.25, 0.25, 1.),
                    ..default()
                })
                .id(),
            cmd.spawn(SpriteBundle {
                texture: textures.gt.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 200.),
                    scale: Vec3::new(0.4, 0.4, 1.),
                    ..default()
                },

                ..default()
            })
            .id(),
            cmd.spawn(self.s2.get_bundle(ma, c_m))
                .insert(Transform {
                    translation: Vec3::new(40., 0., 200.),
                    scale: Vec3::new(0.25, 0.25, 1.),
                    ..default()
                })
                .id(),
        ]
    }
}

pub fn generate_random_goals(amount: usize) -> Vec<Goal> {
    let mut rng = rand::thread_rng();

    repeat_with(|| Goal::random_goal()).take(amount).collect()
}
