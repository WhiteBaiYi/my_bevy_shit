use bevy::prelude::*;
use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
pub struct Spaceship;

impl Plugin for Spaceship {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,load_spaceship);
    }
}

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

fn load_spaceship (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let starting_rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4 * -2.0);

    commands.spawn(
        SpaceshipBundle {
            velocity: Velocity {
                v: Vec3::ZERO,
            },
            model: SceneBundle {
                scene: asset_server.load("Spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION)
            .with_rotation(starting_rotation),
            ..default()
            },
        });
}