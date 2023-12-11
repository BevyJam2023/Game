use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    component_animator_system, Animator, Delay, EaseFunction, Lens, Lerp, Sequence, Tween,
};

use super::hud::{HOVER_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESS_BUTTON_COLOR};
use crate::{
    board::IsOnBoard,
    cards::{goals::Goals, hand::TransformLens, GameState, Score},
    game_shapes::Shape,
    loading::FontAssets,
    AppState,
};
#[derive(Component)]
pub struct MainMenuButton;
#[derive(Component)]
pub struct Scoreboard;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CountUpLens {
    /// Start value.
    pub start: u32,
    /// End value.
    pub end: u32,
}

impl Lens<Text> for CountUpLens {
    fn lerp(&mut self, target: &mut Text, ratio: f32) {
        target.sections[1].value = format!("{}", self.start.lerp(&self.end, &ratio));
    }
}

pub struct ScoreUIPlugin;
impl Plugin for ScoreUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Scoring), (spawn_scoreboard))
            .add_systems(Update, (press_menu).run_if(in_state(GameState::Scoring)))
            .add_systems(Update, component_animator_system::<Text>);
    }
}
pub fn spawn_scoreboard(
    mut cmd: Commands,
    q_goals: Query<&Goals>,
    q_shapes: Query<&Shape, With<IsOnBoard>>,
    fonts: Res<FontAssets>,
    mut score: ResMut<Score>,
) {
    score.base_score = 1000;
    for shape in q_shapes.iter() {
        score.base_score += 1;
    }
    score.score = score.base_score;

    for (i, goal) in q_goals.single().iter().enumerate() {
        let mut lhs = 0;
        let mut rhs = 0;
        for shape in q_shapes.iter() {
            if goal.s1 == *shape {
                lhs += 1;
            }
            if goal.s2 == *shape {
                rhs += 1;
            }
        }

        if lhs > rhs {
            score.goal_status[i] = true;
            score.score *= 2;
        }
    }
    let scoreboard = cmd
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::rgb(153. / 255., 51. / 255., 0.)),
                style: Style {
                    top: Val::Percent(25.0),
                    left: Val::Percent(31.25),
                    width: Val::Percent(37.5),
                    height: Val::Percent(40.),
                    padding: UiRect::all(Val::Px(30.)),

                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(40.),
                    ..default()
                },
                ..default()
            },
            Scoreboard,
        ))
        .with_children(|parent: &mut ChildBuilder| {
            //Title
            parent.spawn((TextBundle {
                style: Style {
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,

                    ..default()
                },
                text: Text {
                    sections: vec![TextSection::new(
                        "Great Job!",
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
            },));
            let total_shapes_tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(3),
                CountUpLens {
                    start: 0,
                    end: score.base_score,
                },
            );
            let goals_achieved_tween = Sequence::new(vec![Delay::new(Duration::from_secs(3))])
                .then(Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs(2),
                    CountUpLens {
                        start: 0,
                        end: score.goal_status.iter().take_while(|&s| *s).count() as u32,
                    },
                ));
            let final_score_tween =
                Sequence::new(vec![Delay::new(Duration::from_secs(5))]).then(Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs(3),
                    CountUpLens {
                        start: 0,
                        end: score.score,
                    },
                ));

            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Total Shapes: ",
                                TextStyle {
                                    font: fonts.fira.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TextSection::new(
                                "",
                                TextStyle {
                                    font: fonts.fira.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            ),
                        ],
                        alignment: TextAlignment::Left,

                        ..default()
                    },

                    ..default()
                },
                Animator::new(total_shapes_tween),
            ));
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Goals Achieved: ",
                                TextStyle {
                                    font: fonts.fira.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TextSection::new(
                                "",
                                TextStyle {
                                    font: fonts.fira.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            ),
                        ],

                        alignment: TextAlignment::Center,

                        ..default()
                    },

                    ..default()
                },
                Animator::new(goals_achieved_tween),
            ));

            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Final Score: ",
                                TextStyle {
                                    font: fonts.fira.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TextSection::new(
                                "",
                                TextStyle {
                                    font: fonts.fira.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            ),
                        ],

                        alignment: TextAlignment::Center,

                        ..default()
                    },

                    ..default()
                },
                Animator::new(final_score_tween),
            ));
            // parent
            //     .spawn((
            //         ButtonBundle {
            //             style: Style {
            //                 width: Val::Px(200.),
            //                 height: Val::Px(80.0),
            //                 justify_content: JustifyContent::Center,
            //                 align_items: AlignItems::Center,
            //                 ..default()
            //             },
            //
            //             background_color: BackgroundColor::from(NORMAL_BUTTON_COLOR),
            //             ..default()
            //         },
            //         MainMenuButton,
            //     ))
            //     .with_children(|parent| {
            //         parent.spawn(TextBundle {
            //             text: Text {
            //                 sections: vec![TextSection::new(
            //                     "Menu",
            //                     TextStyle {
            //                         font: fonts.fira.clone_weak(),
            //                         font_size: 32.0,
            //                         color: Color::BLACK,
            //                     },
            //                 )],
            //                 alignment: TextAlignment::Center,
            //                 ..default()
            //             },
            //             ..default()
            //         });
            //     });
        })
        .id();
}
pub fn despawn_scoreboard(mut cmd: Commands, menu_q: Query<Entity, With<Scoreboard>>) {
    cmd.entity(menu_q.single()).despawn_recursive();
}
pub fn press_menu(
    mut cmd: Commands,
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
) {
    if let Ok((interaction, mut color)) = q_button.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor::from(PRESS_BUTTON_COLOR);
                cmd.insert_resource(NextState(Some(AppState::Menu)));
                cmd.insert_resource(NextState(Some(GameState::Setup)));
            },
            Interaction::Hovered => {
                *color = BackgroundColor::from(HOVER_BUTTON_COLOR);
            },
            Interaction::None => {
                *color = BackgroundColor::from(NORMAL_BUTTON_COLOR);
            },
        }
    }
}
