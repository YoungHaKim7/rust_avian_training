use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(-5.0, 10.0, 20.0))
            .looking_at(Vec3::Y * 0.0, Vec3::Y),
        ..default()
    };
    commands.spawn(camera);
}
