use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
) {
    // this camera will use the default color
    commands.spawn(
        Camera3dBundle { 
                transform: Transform::from_xyz(0.0, 0.0, 5000.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
);
}