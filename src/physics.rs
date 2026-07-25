use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_physics(mut commands: Commands) {
    // Core physics setup - everything lives here
    commands.insert_resource(RapierConfiguration::default());
    // Future: custom gravity, broadphase, etc. for large virtual world
}

// Placeholder for world rules
#[derive(Component)]
pub struct PhysicsWorld;