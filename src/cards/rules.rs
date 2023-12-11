use std::{f32::consts::PI, iter::repeat};

use bevy::prelude::*;
use bevy_tweening::Lerp;

use super::{
    card::{Card, SpawnCard},
    GameState, Score,
};
use crate::{
    operation::{generate_random_operations, Operation},
    AppState,
};

#[derive(Component, Deref, DerefMut)]
pub struct Rule(Vec<Operation>);

#[derive(Event)]
pub struct AddRule {
    pub rule: Operation,
}

pub struct RulePlugin;
impl Plugin for RulePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Playing),
            (spawn_rules).run_if(in_state(GameState::Setup)),
        )
        .add_systems(OnExit(AppState::Playing), reset_rules)
        .add_event::<AddRule>()
        .add_systems(Update, (position_rules, cycle_rule));
    }
}

pub fn spawn_rules(mut cmd: Commands, mut writer: EventWriter<SpawnCard>) {
    let rules_e = cmd
        .spawn((
            Rule(repeat(Operation::None).take(3).collect()),
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(800., -600., 0.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    for _ in 0..3 {
        writer.send(SpawnCard {
            zone_id: rules_e,
            operation: Operation::None,
            face_up: true,
        });
    }
}
pub fn position_rules(
    q_criteria: Query<&Children, With<Rule>>,
    mut q_cards: Query<&mut Transform, With<Card>>,
) {
    if q_criteria.is_empty() {
        return;
    }
    let children = q_criteria.single();
    for (i, &entity) in children.iter().enumerate() {
        if let Ok(mut transform) = q_cards.get_mut(entity) {
            transform.translation.x = transform.translation.x.lerp(&(i as f32 * 150.), &0.2);
            transform.translation.y = transform.translation.y.lerp(&0., &0.2);

            transform.translation.z = 20.;
            transform.rotation = transform
                .rotation
                .lerp(Quat::from_euler(EulerRot::XYZ, 0., PI, 0.), 0.2);
        }
    }
}
pub fn cycle_rule(
    mut cmd: Commands,
    mut q_rules: Query<(&mut Rule, &mut Children)>,
    mut reader: EventReader<AddRule>,
    mut score: ResMut<Score>,
) {
    for event in reader.read() {
        let (mut rule, mut children) = q_rules.single_mut();
        dbg!(rule.len());

        if rule.len() >= 3 {
            score.cards_played += 1;
            rule.remove(2);
            cmd.entity(*children.iter().last().unwrap()).remove_parent();

            cmd.entity(*children.iter().last().unwrap())
                .despawn_recursive();
            rule.insert(0, event.rule.clone());
        }
    }
}
pub fn reset_rules(mut cmd: Commands, q_rule: Query<Entity, With<Rule>>) {
    cmd.entity(q_rule.single()).despawn_recursive();
}
