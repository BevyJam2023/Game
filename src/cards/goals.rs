use bevy::{prelude::*, render::view::RenderLayers};
use bevy_tweening::Lerp;

use super::{
    card::{Card, SpawnCard, SpawnGoalCard},
    GameState,
};
use crate::{
    goal::{self, generate_random_goals, Goal},
    operation::{generate_random_operations, Operation},
    AppState,
};

#[derive(Component, Deref, DerefMut)]
pub struct Goals(Vec<Goal>);

pub struct GoalsPlugin;
impl Plugin for GoalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Playing),
            (spawn_goals).run_if(in_state(GameState::Setup)),
        )
        .add_systems(OnExit(AppState::Playing), reset_goals)
        .add_systems(Update, (position_goals));
    }
}

pub fn spawn_goals(mut cmd: Commands, mut writer: EventWriter<SpawnGoalCard>) {
    let goals = generate_random_goals(3);
    dbg!(goals.clone());
    let rules_e = cmd
        .spawn((
            Goals(goals.clone()),
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(-1100., -600., 0.),
                    ..default()
                },
                ..default()
            },
            RenderLayers::layer(1),
        ))
        .id();
    for g in goals {
        writer.send(SpawnGoalCard {
            zone_id: rules_e,
            goal: g,
            face_up: true,
        });
    }
}
pub fn position_goals(
    q_criteria: Query<&Children, With<Goals>>,
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
        }
    }
}
pub fn reset_goals(mut cmd: Commands, q_goals: Query<Entity, With<Goals>>) {
    cmd.entity(q_goals.single()).despawn_recursive();
}
