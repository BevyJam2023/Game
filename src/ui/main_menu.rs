use bevy::prelude::*;

use crate::{
    loading::{FontAssets, SoundAssets, TextureAssets},
    AppState,
};
pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(1., 1., 1.);
pub const HOVER_BUTTON_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PRESS_BUTTON_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component)]
pub struct MainMenu;
#[derive(Component)]
pub struct PlayButton;
#[derive(Component)]
pub struct InstructionButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), (spawn_main_menu))
            .add_systems(
                Update,
                (press_instruction, press_play).run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnExit(AppState::Menu), (despawn_main_menu));
    }
}
pub fn spawn_main_menu(
    mut cmd: Commands,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
    sound: Res<SoundAssets>,
) {
    cmd.spawn(AudioBundle {
        source: sound.bg_music.clone(),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            ..default()
        },
    });
    let main_menu = cmd
        .spawn((
            NodeBundle {
                background_color: Color::rgb(153. / 255., 173. / 255., 211. / 255.).into(),
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
            MainMenu,
        ))
        .with_children(|parent: &mut ChildBuilder| {
            //Title
            parent.spawn(
                (TextBundle {
                    style: Style {
                        padding: UiRect {
                            bottom: Val::Px(100.),
                            ..default()
                        },
                        ..default()
                    },
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Shape",
                                TextStyle {
                                    font: fonts.fira.clone(),
                                    font_size: 100.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TextSection::new(
                                "craft",
                                TextStyle {
                                    font: fonts.fira.clone(),
                                    font_size: 100.0,
                                    color: Color::WHITE,
                                },
                            ),
                        ],
                        alignment: TextAlignment::Center,

                        ..default()
                    },

                    ..default()
                }),
            );

            //Play Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.),
                            height: Val::Px(80.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },

                        background_color: BackgroundColor::from(NORMAL_BUTTON_COLOR),
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                TextStyle {
                                    font: fonts.fira.clone_weak(),
                                    font_size: 32.0,
                                    color: Color::BLACK,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            // //Instructions Button
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
            //         InstructionButton,
            //     ))
            //     .with_children(|parent| {
            //         parent.spawn(TextBundle {
            //             text: Text {
            //                 sections: vec![TextSection::new(
            //                     "Instructions",
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
            parent.spawn(ImageBundle {
                image: textures.card_blue.clone().into(),

                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    ..default()
                },
                ..default()
            });

            parent.spawn(ImageBundle {
                image: textures.card_red.clone().into(),

                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    right: Val::Px(0.),
                    ..default()
                },
                ..default()
            });

            parent.spawn(ImageBundle {
                image: textures.card_blue.clone().into(),

                style: Style {
                    bottom: Val::Px(0.),
                    right: Val::Px(0.),
                    position_type: PositionType::Absolute,

                    ..default()
                },
                ..default()
            });

            parent.spawn(ImageBundle {
                image: textures.card_red.clone().into(),

                style: Style {
                    bottom: Val::Px(0.),
                    left: Val::Px(0.),
                    position_type: PositionType::Absolute,

                    ..default()
                },
                ..default()
            });
        })
        .id();
}
pub fn despawn_main_menu(mut cmd: Commands, menu_q: Query<Entity, With<MainMenu>>) {
    cmd.entity(menu_q.single()).despawn_recursive();
}
pub fn press_play(
    mut cmd: Commands,
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
) {
    if let Ok((interaction, mut color)) = q_button.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor::from(PRESS_BUTTON_COLOR);
                cmd.insert_resource(NextState(Some(AppState::Playing)));
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
pub fn press_instruction(
    mut cmd: Commands,
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<InstructionButton>),
    >,
) {
    if let Ok((interaction, mut color)) = q_button.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor::from(PRESS_BUTTON_COLOR);
                cmd.insert_resource(NextState(Some(AppState::Instruction)));
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
