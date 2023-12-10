use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::ScalingMode, view::RenderLayers},
    window::PrimaryWindow,
};
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::{board, AppState};

#[derive(Debug, Component)]
pub struct CardCamera;
#[derive(Debug, Component)]
pub struct BoardCamera;

#[derive(Component)]
pub struct CameraFollow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), setup);
        app.add_systems(Update, free_cam_movement);
        app.add_plugins(PanCamPlugin::default());
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                // no "background color", we need to see the main camera's output
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            camera: Camera {
                order: 1,
                ..default()
            },
            projection: OrthographicProjection {
                near: -1000.,
                scaling_mode: ScalingMode::AutoMin {
                    min_width: 1.5 * board::config::SIZE.0,
                    min_height: 1.5 * board::config::SIZE.1,
                },
                // scale: 2.,
                ..Default::default()
            },
            ..default()
        },
        CardCamera,
        RenderLayers::layer(0),
    ));
}

pub fn lerp(x: f32, y: f32, by: f32) -> f32 {
    x * (1. - by) + y * by
}

// A simple camera system for moving and zooming the camera.
#[allow(dead_code)]
pub fn free_cam_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
