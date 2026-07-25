//! Future Linux emulator module.
//!
//! CRITICAL RULE: The emulator NEVER owns hardware entities.
//! Hardware lives independently in the physics world.
//! The emulator only observes and writes through the public bus / interface
//! components defined in the hardware module.

use bevy::prelude::*;
use crate::hardware::{VirtualBus, MemoryMappedIo, InterruptLine, BusParticipant};

/// Marker component for the emulator process itself.
/// This is a pure software entity – it has no physics body and does not
/// parent or own any hardware.
#[derive(Component)]
pub struct EmulatorProcess;

/// Systems that will later implement the emulator loop.
/// They query the bus components; they never take ownership of hardware.

pub fn emulator_tick(
    // Read-only access to buses that hardware has published
    buses: Query<&VirtualBus, With<BusParticipant>>,
    // Writable access only to the interface registers, never to the hardware entities themselves
    mut mmio: Query<&mut MemoryMappedIo>,
    mut interrupts: Query<&mut InterruptLine>,
) {
    // Placeholder: future emulator code will:
    // 1. Read memory / registers via MemoryMappedIo
    // 2. Observe InterruptLine events
    // 3. Write back results to the same interfaces
    // Hardware continues to exist and simulate independently in the physics world.
    let _ = (buses, mmio, interrupts);
}

/// Plugin that will be registered later when the emulator is implemented.
/// It adds systems that talk to the hardware interface layer only.
pub struct EmulatorPlugin;

impl Plugin for EmulatorPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, emulator_tick);
        // Currently a no-op placeholder so the architecture is clear.
    }
}
