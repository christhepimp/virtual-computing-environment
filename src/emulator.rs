//! Future Linux emulator.
//!
//! The emulator is software that runs *on* the virtual computer.
//! Because the virtual computer exists entirely inside the physics world,
//! the emulator itself is also part of that world.
//!
//! It does not create, own, or simulate any hardware.
//! It only observes and communicates through the interfaces
//! (VirtualBus, MemoryMappedIo, InterruptLine, …) that the hardware
//! entities themselves expose.
//!
//! This keeps a single source of truth: the physics simulation.

use bevy::prelude::*;
use crate::hardware::{VirtualBus, MemoryMappedIo, InterruptLine, BusParticipant};

/// Marker for a software process that is running on the virtual computer.
/// This entity has no physics body of its own; its existence is predicated
/// on the hardware being present and powered inside the physics world.
#[derive(Component)]
pub struct EmulatorProcess;

/// Placeholder tick. Future implementation will:
/// 1. Read registers and memory through MemoryMappedIo
/// 2. Observe InterruptLine events raised by hardware
/// 3. Write results back through the same interfaces
///
/// At no point does this code own or directly mutate hardware entities.
pub fn emulator_tick(
    buses: Query<&VirtualBus, With<BusParticipant>>,
    mut mmio: Query<&mut MemoryMappedIo>,
    mut interrupts: Query<&mut InterruptLine>,
) {
    // Intentionally empty for now – architecture only.
    let _ = (buses, mmio, interrupts);
}

/// Plugin that will later register the emulator systems.
/// Even when fully implemented it will only ever touch interface components.
pub struct EmulatorPlugin;

impl Plugin for EmulatorPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, emulator_tick);
        // Left disabled until the first real emulator logic is introduced.
    }
}
