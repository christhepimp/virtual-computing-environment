//! Future Linux emulator.
//!
//! The emulator runs as software *inside* the virtual computer.
//! It never owns, creates, or simulates hardware.
//!
//! All interaction happens through the world systems and the interfaces
//! that hardware entities expose:
//!   - PowerSystem, ClockSystem
//!   - DeviceRegistry
//!   - BusSystem, SignalSystem
//!   - MemoryMapSystem, InterruptSystem
//!   - ConnectionSystem
//!
//! This module remains a non-owning placeholder until a later stage.

use bevy::prelude::*;

use crate::world::buses::BusSystem;
use crate::world::memory::MemoryMapSystem;
use crate::world::interrupts::InterruptSystem;
use crate::world::devices::DeviceRegistry;
use crate::world::power::PowerSystem;
use crate::world::clock::ClockSystem;

/// Marker for a software process running on the virtual computer.
#[derive(Component)]
pub struct EmulatorProcess;

/// Placeholder tick. Future implementation will only touch world systems
/// and published interfaces — never hardware entities directly as owner.
pub fn emulator_tick(
    _power: Res<PowerSystem>,
    _clock: Res<ClockSystem>,
    _registry: Res<DeviceRegistry>,
    _buses: Res<BusSystem>,
    _memory: Res<MemoryMapSystem>,
    _interrupts: ResMut<InterruptSystem>,
) {
    // Intentionally empty — architecture only.
}

pub struct EmulatorPlugin;

impl Plugin for EmulatorPlugin {
    fn build(&self, _app: &mut App) {
        // Will later add: app.add_systems(Update, emulator_tick);
    }
}
