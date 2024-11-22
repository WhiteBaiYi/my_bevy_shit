use bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
pub struct Spaceship;

impl Plugin for Spaceship {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,load_spaceship);
    }
}

fn load_spaceship (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let starting_rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4 * -2.0); // Rotate 45 degrees around the y-axis
    commands.spawn(
             SceneBundle {
                scene: asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("Spaceship.glb#Scene0")),
                transform: Transform::from_translation(STARTING_TRANSLATION).with_rotation(starting_rotation),
                ..default()
        });
}