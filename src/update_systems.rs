use crate::{components::*, constants::*};
use bevy::prelude::*;

pub fn camera_watch_cue_ball(
    camera: Single<&mut Transform, With<Camera3d>>,
    cue_ball: Single<(&Transform, &Velocity), (With<CueBall>, Without<Camera3d>)>,
    time: Res<Time>,
) {
    let (cue_ball_transform, cue_ball_velocity) = cue_ball.into_inner();
    let mut camera_transform = camera.into_inner();

    camera_transform.look_at(cue_ball_transform.translation, Vec3::Y);

    if cue_ball_velocity.speed > 0.0 {
        camera_transform.translation +=
            cue_ball_velocity.direction * cue_ball_velocity.speed * time.delta_secs();
    }
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

// when Space key is pressed set a velocity for the cue ball based on the relative position of the camera
pub fn hit_intraction(
    keys: Res<ButtonInput<KeyCode>>,
    camera: Single<&Transform, With<Camera3d>>,
    cue_ball: Single<(&Transform, &mut Velocity), (With<CueBall>, Without<Camera3d>)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let (cue_ball_transform, mut cue_ball_velocity) = cue_ball.into_inner();
        let camera_transformation = camera.into_inner();

        let mut direction = cue_ball_transform.translation - camera_transformation.translation;
        // set y to 0 since the camera will be above the ball but we only want to ball to run along the table
        direction.y = 0.0;
        cue_ball_velocity.direction = direction;
        cue_ball_velocity.speed = 1.0;
    }
}

pub fn enact_velocity(velocity_entities: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    for (mut transform, mut velocity) in velocity_entities {
        if velocity.speed > 0.0 {
            transform.translation += velocity.direction * velocity.speed * time.delta_secs();
            velocity.speed -= DECELERATION;
            if velocity.speed < 0.0 {
                velocity.speed = 0.0;
            }
        }
    }
}
