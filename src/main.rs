use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Ground
    commands.spawn((
        Collider::cuboid(50.0, 0.1, 50.0),
        RigidBody::Fixed,
        Transform::from_xyz(0.0, -0.1, 0.0),
    ));

    println!("Virtual Computing Environment initialized. Physics world ready.");
}
