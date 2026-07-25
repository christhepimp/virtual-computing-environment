use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use virtual_computing_environment::{
    setup_physics, setup_room, spawn_virtual_computer, VirtualComputerPlugin,
};
use virtual_computing_environment::world::power::{PowerEvent, PowerSystem};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Virtual Computing Environment".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(VirtualComputerPlugin)
        .add_systems(
            Startup,
            (
                setup,
                setup_physics,
                setup_room,
                spawn_virtual_computer,
                apply_power_on,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-15.0, 8.0, 15.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.6, 0.5, 0.7),
        brightness: 0.3,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    println!("Virtual Computing Environment");
    println!("Physics world is the active runtime of the virtual computer.");
}

/// Foundation convenience: apply main power after devices spawn so firmware runs.
fn apply_power_on(mut power: ResMut<PowerSystem>, mut events: EventWriter<PowerEvent>) {
    power.set_main_power(true, &mut events);
    println!("Main power applied — firmware will initialize the machine.");
}
