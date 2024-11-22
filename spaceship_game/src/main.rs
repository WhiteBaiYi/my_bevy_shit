mod camera;
mod spaceship;
mod movement;
mod debug;

use bevy::prelude::*;
use camera::setup;
use spaceship::Spaceship;
use movement::MovementPlugin;
use debug::print_position;

fn main () {
    App::new()
        .insert_resource(ClearColor(
            Color::srgba(0.2, 0.23, 0.29, 1.0))
        )
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(Spaceship)
        .add_plugins(MovementPlugin)
        .add_systems(Update,print_position)
        .add_systems(Startup,setup )
        .run();
}