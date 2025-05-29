mod setup_systems;
use setup_systems::*;

use bevy::{
    DefaultPlugins,
    app::{App, Plugin, Startup},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PooltablePlugin))
        .run();
}

pub struct PooltablePlugin;

impl Plugin for PooltablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_table, add_camera, add_light));
    }
}
