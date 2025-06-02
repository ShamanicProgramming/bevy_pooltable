use crate::{components::*, constants::CAMERA_ROTATION_SPEED};
use bevy::prelude::*;

pub fn camera_watch_cue_ball(
    camera: Single<&mut Transform, With<Camera3d>>,
    cue_ball: Single<&Transform, (With<CueBall>, Without<Camera3d>)>,
) {
    let cue_ball_transform = cue_ball.into_inner();
    let mut camera_transform = camera.into_inner();

    camera_transform.look_at(cue_ball_transform.translation, Vec3::Y);
}

pub fn rotate_camera_interaction(
    keys: Res<ButtonInput<KeyCode>>,
    camera: Single<&mut Transform, With<Camera3d>>,
    cue_ball: Single<&Transform, (With<CueBall>, Without<Camera3d>)>,
    time: Res<Time>,
) {
    let cue_ball_transform = cue_ball.into_inner();
    let mut camera_transformation = camera.into_inner();
    if keys.pressed(KeyCode::KeyE) {
        camera_transformation.rotate_around(
            cue_ball_transform.translation,
            Quat::from_rotation_y(CAMERA_ROTATION_SPEED * time.delta_secs()),
        );
    } else if keys.pressed(KeyCode::KeyQ) {
        camera_transformation.rotate_around(
            cue_ball_transform.translation,
            Quat::from_rotation_y(-CAMERA_ROTATION_SPEED * time.delta_secs()),
        );
    }
}
