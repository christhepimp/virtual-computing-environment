//! Future Linux emulator — non-owning guest software.
//!
//! Starts only when FirmwarePhase::Ready. Uses bus transactions, MMIO,
//! interrupts, storage blocks, and power/clock signals exclusively.

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
    // Guest OS entry point will live here.
}

pub struct EmulatorPlugin;

impl Plugin for EmulatorPlugin {
    fn build(&self, _app: &mut App) {}
}
