use bevy::{input::keyboard, log::info, math::Vec3, prelude::{App, ButtonInput, Component, KeyCode, Plugin, Query, Res, Time, Transform, Update}};

#[derive(Component)]
pub struct Velocity {
    pub v: Vec3,
}
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

fn movement (
    mut query: Query<(&Velocity,&mut Transform)>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    for (
        velocity, 
        mut transform
    ) in query.iter_mut() {
    
    let mut direction = Vec3::ZERO;

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.just_pressed(KeyCode::KeyS)
{
        direction -= Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);

    }
    transform.translation += direction * velocity.v * time.delta_seconds();
    }
}