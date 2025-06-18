use crate::{components::*, constants::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{ExternalImpulse, Velocity};

/// Matches the camera velocity to the cue ball velocity
pub fn follow_cam(
    camera: Single<&mut Velocity, (With<Camera3d>, Without<CueBall>)>,
    cue_ball: Single<&Velocity, (With<CueBall>, Without<Camera3d>)>,
) {
    let mut camera_velocity = camera.into_inner();
    let cue_ball_velocity = cue_ball.into_inner();

    camera_velocity.linvel = cue_ball_velocity.linvel.clone();
}

/// Rotates the camera around the cue ball when the E or Q keys are pressed.
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

/// When the Space key is pressed, sets an impulse for the cue ball based on the relative position of the camera.
pub fn hit_intraction(
    keys: Res<ButtonInput<KeyCode>>,
    camera: Single<&Transform, With<Camera3d>>,
    cue_ball: Single<(&Transform, &mut ExternalImpulse), With<CueBall>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let (cue_ball_transform, mut cue_ball_impulse) = cue_ball.into_inner();
        let camera_transform = camera.into_inner();

        let mut direction = cue_ball_transform.translation - camera_transform.translation;
        // set y to 0 since the camera will be above the ball but we only want to ball to run along the table
        direction.y = 0.0;
        let normalized_direction = direction.normalize();

        cue_ball_impulse.impulse = normalized_direction * BALL_HIT_MAGNITUDE;
    }
}
