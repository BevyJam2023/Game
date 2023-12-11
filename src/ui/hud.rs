use bevy::prelude::*;

use super::StartText;
use crate::{
    cards::GameTimer,
    loading::{FontAssets, TextureAssets},
    AppState,
};
pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(1., 1., 1.);
pub const HOVER_BUTTON_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PRESS_BUTTON_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component)]
pub struct HUD;
#[derive(Component)]
pub struct TimerText;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), (spawn_hud))
            .add_systems(
                Update,
                (update_timer_text).run_if(in_state(AppState::Playing)),
            )
            .add_systems(OnExit(AppState::Playing), (despawn_hud));
    }
}
pub fn spawn_hud(mut cmd: Commands, fonts: Res<FontAssets>) {
    let hud = cmd
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(40.),
                    ..default()
                },
                ..default()
            },
            HUD,
        ))
        .with_children(|parent: &mut ChildBuilder| {
            //Title
            parent.spawn((
                TextBundle {
                    style: Style {
                        top: Val::Px(30.),
                        left: Val::Px(30.),
                        position_type: PositionType::Absolute,

                        ..default()
                    },
                    text: Text {
                        sections: vec![TextSection::new(
                            "1:00",
                            TextStyle {
                                font: fonts.fira.clone(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        )],
                        alignment: TextAlignment::Center,

                        ..default()
                    },

                    ..default()
                },
                TimerText,
            ));
            parent.spawn((
                TextBundle {
                    style: Style {
                        top: Val::Px(30.),
                        right: Val::Px(100.),
                        position_type: PositionType::Absolute,

                        ..default()
                    },
                    text: Text {
                        sections: vec![TextSection::new(
                            "Goals",
                            TextStyle {
                                font: fonts.fira.clone(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        )],
                        alignment: TextAlignment::Center,

                        ..default()
                    },

                    ..default()
                },
                TimerText,
            ));
            parent.spawn((
                TextBundle {
                    style: Style {
                        top: Val::Px(200.),
                        right: Val::Px(100.),
                        position_type: PositionType::Absolute,

                        ..default()
                    },
                    text: Text {
                        sections: vec![TextSection::new(
                            "Rules",
                            TextStyle {
                                font: fonts.fira.clone(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        )],
                        alignment: TextAlignment::Center,

                        ..default()
                    },

                    ..default()
                },
                TimerText,
            ));

            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text {
                        sections: vec![TextSection::new(
                            "Press Space to Start",
                            TextStyle {
                                font: fonts.fira.clone(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        )],
                        alignment: TextAlignment::Center,

                        ..default()
                    },

                    ..default()
                },
                StartText,
            ));
        })
        .id();
}
pub fn despawn_hud(mut cmd: Commands, menu_q: Query<Entity, With<HUD>>) {
    cmd.entity(menu_q.single()).despawn_recursive();
}
pub fn update_timer_text(
    mut cmd: Commands,
    game_timer: Res<GameTimer>,
    mut q_text: Query<(&mut Text), (With<TimerText>)>,
) {
    if let Ok(mut text) = q_text.get_single_mut() {
        text.sections[0].value =
            (120 - game_timer.timer.elapsed_secs().round() as u32).to_string() + "s";
    }
}
