//! Firmware (BIOS/UEFI equivalent).
//!
//! Runs before any guest software. Discovers hardware via the DeviceRegistry,
//! initializes power and clocks, sets up the memory map view, and brings the
//! machine to a state where an OS can boot.
//!
//! Firmware is software that lives inside the virtual computer — it does not
//! own hardware; it uses the same world interfaces an OS would use.

use bevy::prelude::*;

use super::clock::ClockSystem;
use super::devices::{DeviceKind, DeviceRegistry};
use super::memory::MemoryMapSystem;
use super::power::{PowerEvent, PowerSystem};
use super::signals::{SignalId, SignalSystem};
use super::storage::StorageSystem;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirmwarePhase {
    Off,
    PowerOnSelfTest,
    DeviceDiscovery,
    MemoryInit,
    StorageInit,
    Ready,
    Halted,
}

#[derive(Resource)]
pub struct Firmware {
    pub phase: FirmwarePhase,
    pub post_passed: bool,
    pub boot_device: Option<Entity>,
}

impl Default for Firmware {
    fn default() -> Self {
        Self {
            phase: FirmwarePhase::Off,
            post_passed: false,
            boot_device: None,
        }
    }
}

/// Drive the firmware state machine when main power is applied.
pub fn firmware_tick(
    mut fw: ResMut<Firmware>,
    mut power: ResMut<PowerSystem>,
    mut clock: ResMut<ClockSystem>,
    mut signals: ResMut<SignalSystem>,
    registry: Res<DeviceRegistry>,
    memory: Res<MemoryMapSystem>,
    storage: Res<StorageSystem>,
    mut power_events: EventReader<PowerEvent>,
) {
    for ev in power_events.read() {
        match ev {
            PowerEvent::MainPowerOn => {
                if fw.phase == FirmwarePhase::Off || fw.phase == FirmwarePhase::Halted {
                    fw.phase = FirmwarePhase::PowerOnSelfTest;
                    println!("[Firmware] Power-on — starting POST");
                }
            }
            PowerEvent::MainPowerOff => {
                fw.phase = FirmwarePhase::Off;
                fw.post_passed = false;
                fw.boot_device = None;
                signals.deassert(SignalId::PowerGood);
                signals.deassert(SignalId::ClockEnable);
                println!("[Firmware] Power-off — halted");
            }
            _ => {}
        }
    }

    if !power.main_power {
        return;
    }

    match fw.phase {
        FirmwarePhase::PowerOnSelfTest => {
            // Minimal POST: require motherboard + CPU + RAM present.
            let has_mb = !registry.devices_of_kind(DeviceKind::Motherboard).is_empty();
            let has_cpu = !registry.devices_of_kind(DeviceKind::Cpu).is_empty();
            let has_ram = !registry.devices_of_kind(DeviceKind::Ram).is_empty();

            if has_mb && has_cpu && has_ram {
                fw.post_passed = true;
                fw.phase = FirmwarePhase::DeviceDiscovery;
                println!("[Firmware] POST passed");
            } else {
                fw.phase = FirmwarePhase::Halted;
                println!("[Firmware] POST failed — missing critical devices");
            }
        }
        FirmwarePhase::DeviceDiscovery => {
            println!("[Firmware] Devices discovered: {}", registry.devices.len());
            for (entity, info) in registry.devices.iter() {
                // Power on devices that are part of the machine.
                power.set_device_power(*entity, true);
                if let Some(dc) = clock.device_clocks.get_mut(entity) {
                    dc.enabled = true;
                }
                println!("  - {} ({:?})", info.name, info.kind);
            }
            fw.phase = FirmwarePhase::MemoryInit;
        }
        FirmwarePhase::MemoryInit => {
            println!("[Firmware] Memory map regions: {}", memory.regions.len());
            for (_, region) in memory.regions.iter() {
                println!(
                    "  - {:#x}..{:#x} {} (ram={})",
                    region.base,
                    region.base + region.size,
                    region.name,
                    region.is_ram
                );
            }
            fw.phase = FirmwarePhase::StorageInit;
        }
        FirmwarePhase::StorageInit => {
            println!("[Firmware] Block devices: {}", storage.devices.len());
            for (entity, dev) in storage.devices.iter() {
                println!("  - {:?}  {} sectors", entity, dev.sectors);
                if fw.boot_device.is_none() {
                    fw.boot_device = Some(*entity);
                }
            }
            // Assert power-good and clock-enable — machine ready for software.
            signals.assert(SignalId::PowerGood);
            signals.assert(SignalId::ClockEnable);
            fw.phase = FirmwarePhase::Ready;
            println!("[Firmware] Machine ready. Awaiting guest software.");
        }
        FirmwarePhase::Ready | FirmwarePhase::Off | FirmwarePhase::Halted => {}
    }
}
