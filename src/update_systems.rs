use crate::components::*;
use bevy::prelude::*;

pub fn camera_watch_cue_ball(
    camera: Single<&mut Transform, With<Camera3d>>,
    cue_ball: Single<&Transform, (With<CueBall>, Without<Camera3d>)>,
) {
    let cue_ball_transform = cue_ball.into_inner();
    let mut camera_transform = camera.into_inner();

    camera_transform.look_at(cue_ball_transform.translation, Vec3::Y);
}
