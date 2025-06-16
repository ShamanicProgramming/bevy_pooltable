mod components;
mod constants;
mod setup_systems;
mod update_systems;
use bevy_rapier3d::prelude::*;
use setup_systems::*;
use update_systems::*;

use bevy::{
    DefaultPlugins,
    app::{App, Plugin, Startup, Update},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PooltablePlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}

pub struct PooltablePlugin;

impl Plugin for PooltablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_table, add_camera, add_light, add_balls));
        app.add_systems(
            Update,
            (
                rotate_camera_interaction,
                hit_intraction,
                apply_deceleration,
                follow_cam,
            ),
        );
    }
}
