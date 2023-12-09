use std::iter::repeat_with;

use bevy::{
    prelude::{Entity, Handle, Image, ResMut},
    sprite::SpriteBundle,
    utils::default,
};
use rand::{seq::IteratorRandom, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{game_shapes::Shape, loading::TextureAssets};
#[derive(Clone)]
pub enum Operation {
    Mul(Shape, u32),
    Sub(Shape, Shape),
    Add(Shape, Shape),
    Exp(Shape, u32),
}
impl Operation {
    // pub fn get_operation_asset(textures: ResMut<TextureAssets>) -> Handle<Image> {
    //     match  {
    //
    //     }
    // }
    pub fn random_operation() -> Operation {
        let mut rng = rand::thread_rng();

        let o = rng.gen_range(0..4);
        match o {
            0 => Operation::Mul(Shape::random_shape(), 2),
            1 => Operation::Sub(Shape::random_shape(), Shape::random_shape()),
            2 => Operation::Add(Shape::random_shape(), Shape::random_shape()),
            _ => Operation::Exp(Shape::random_shape(), 2),
        }
    }

    pub(crate) fn get_operation_entity(
        &self,
        cmd: &mut bevy::prelude::Commands<'_, '_>,
    ) -> [Entity; 1] {
        match self {
            Operation::Mul(s, i) => {
                // s.get_sprite();
            },
            Operation::Sub(s1, s2) => {},
            Operation::Add(s1, s2) => {},
            Operation::Exp(s, i) => {},
        }
        [cmd.spawn(SpriteBundle { ..default() }).id()]
    }
}
pub fn generate_random_operations(num: usize) -> Vec<Operation> {
    let mut rng = rand::thread_rng();

    repeat_with(|| Operation::random_operation())
        .take(num)
        .collect()
}
