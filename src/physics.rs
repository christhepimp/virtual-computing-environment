//! The physics engine is the single source of truth for the entire virtual world.
//!
//! Everything that exists — the room, the computer, cables, future research
//! apparatus — exists because it is part of this simulation. No subsystem
//! is allowed to define a parallel reality outside of it.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Marker / configuration for the world itself.
/// All other entities inhabit this reality.
#[derive(Component)]
pub struct PhysicsWorld;

pub fn setup_physics(mut commands: Commands) {
    // The physics configuration is the root of the virtual reality.
    // Future work may tune gravity, solver iterations, broad-phase, etc.
    // to support large or unusual experimental setups.
    commands.insert_resource(RapierConfiguration::default());

    println!("Physics world is the single source of truth. All objects exist inside it.");
}
