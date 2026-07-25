//! Future Linux emulator.
//!
//! Runs as software inside the virtual computer. Communicates only through:
//!   - Bus transactions (BusSystem)
//!   - Memory map / RAM (MemoryMapSystem)
//!   - Interrupts (InterruptSystem)
//!   - Power / clock / signals
//!   - Device registry (discovery)
//!   - Storage block interface (StorageSystem)
//!
//! Never owns or simulates hardware.

use bevy::prelude::*;

use crate::world::buses::BusSystem;
use crate::world::clock::ClockSystem;
use crate::world::devices::DeviceRegistry;
use crate::world::firmware::{Firmware, FirmwarePhase};
use crate::world::interrupts::InterruptSystem;
use crate::world::memory::MemoryMapSystem;
use crate::world::power::PowerSystem;
use crate::world::storage::StorageSystem;

#[derive(Component)]
pub struct EmulatorProcess;

/// Placeholder: when firmware reports Ready, guest software may start.
pub fn emulator_tick(
    firmware: Res<Firmware>,
    _power: Res<PowerSystem>,
    _clock: Res<ClockSystem>,
    _registry: Res<DeviceRegistry>,
    _buses: ResMut<BusSystem>,
    _memory: ResMut<MemoryMapSystem>,
    _interrupts: ResMut<InterruptSystem>,
    _storage: ResMut<StorageSystem>,
) {
    if firmware.phase != FirmwarePhase::Ready {
        return;
    }
    // Future: fetch instructions / drive OS via bus + MMIO + IRQs only.
}

pub struct EmulatorPlugin;

impl Plugin for EmulatorPlugin {
    fn build(&self, _app: &mut App) {}
}
