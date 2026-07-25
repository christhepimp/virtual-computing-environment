//! The room and atmosphere also exist inside the physics world.
//! They are not a backdrop painted on top of the simulation;
//! they are physical (or physically influenced) elements of the same reality.

use bevy::prelude::*;

/// Eerie room inspired by the atmosphere of a lonely, outdated computer room.
/// Original design — dim light, long shadows, quiet tension.
pub fn setup_room(_commands: Commands, _asset_server: Res<AssetServer>) {
    // TODO: walls, floor, ceiling as physics-aware geometry
    // TODO: dim, slightly unnatural lighting
    // TODO: dust, cables, small props that can be interacted with
    // All of these will be entities inside the same physics world as the computer.

    println!("Eerie room exists inside the physics world.");
}
