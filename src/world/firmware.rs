//! Firmware (BIOS/UEFI equivalent).
//!
//! Runs after PowerGood. Discovers hardware, drives init policy, and only
//! declares the machine Ready when critical devices report Ready through
//! their independent state machines.

use bevy::prelude::*;

use super::devices::{DeviceKind, DeviceRegistry, Registered};
use super::lifecycle::{DeviceLifecycle, DevicePhase};
use super::memory::MemoryMapSystem;
use super::power::{PowerEvent, PowerSystem};
use super::signals::{SignalId, SignalSystem};
use super::storage::StorageSystem;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirmwarePhase {
    Off,
    WaitPowerGood,
    PowerOnSelfTest,
    DeviceDiscovery,
    MemoryInit,
    StorageInit,
    DeviceReadyWait,
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

pub fn firmware_tick(
    mut fw: ResMut<Firmware>,
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    registry: Res<DeviceRegistry>,
    memory: Res<MemoryMapSystem>,
    storage: Res<StorageSystem>,
    life_query: Query<(Entity, &DeviceLifecycle), With<Registered>>,
    mut power_events: EventReader<PowerEvent>,
) {
    for ev in power_events.read() {
        match ev {
            PowerEvent::MainPowerOn => {
                fw.phase = FirmwarePhase::WaitPowerGood;
                fw.post_passed = false;
                fw.boot_device = None;
                println!("[Firmware] Main power on — waiting for PowerGood");
            }
            PowerEvent::MainPowerOff => {
                fw.phase = FirmwarePhase::Off;
                fw.post_passed = false;
                fw.boot_device = None;
                println!("[Firmware] Main power off");
            }
            _ => {}
        }
    }

    if !power.main_power {
        if fw.phase != FirmwarePhase::Off {
            fw.phase = FirmwarePhase::Off;
        }
        return;
    }

    match fw.phase {
        FirmwarePhase::WaitPowerGood => {
            if signals.is_asserted(SignalId::PowerGood) {
                fw.phase = FirmwarePhase::PowerOnSelfTest;
                println!("[Firmware] PowerGood received — starting POST");
            }
        }
        FirmwarePhase::PowerOnSelfTest => {
            let has_mb = !registry.devices_of_kind(DeviceKind::Motherboard).is_empty();
            let has_cpu = !registry.devices_of_kind(DeviceKind::Cpu).is_empty();
            let has_ram = !registry.devices_of_kind(DeviceKind::Ram).is_empty();
            let has_psu = !registry.devices_of_kind(DeviceKind::PowerSupply).is_empty();

            if has_mb && has_cpu && has_ram && has_psu {
                fw.post_passed = true;
                fw.phase = FirmwarePhase::DeviceDiscovery;
                println!("[Firmware] POST passed");
            } else {
                fw.phase = FirmwarePhase::Halted;
                println!("[Firmware] POST failed — missing critical devices");
            }
        }
        FirmwarePhase::DeviceDiscovery => {
            println!("[Firmware] Devices in registry: {}", registry.devices.len());
            for (_e, info) in registry.devices.iter() {
                println!("  - {} ({:?})", info.name, info.kind);
            }
            fw.phase = FirmwarePhase::MemoryInit;
        }
        FirmwarePhase::MemoryInit => {
            println!("[Firmware] Memory regions: {}", memory.regions.len());
            for (_, r) in memory.regions.iter() {
                println!(
                    "  - {:#x}+{:#x} {} ram={}",
                    r.base, r.size, r.name, r.is_ram
                );
            }
            fw.phase = FirmwarePhase::StorageInit;
        }
        FirmwarePhase::StorageInit => {
            println!("[Firmware] Block devices: {}", storage.devices.len());
            for (entity, dev) in storage.devices.iter() {
                println!("  - {:?} ({} sectors)", entity, dev.sectors);
                if fw.boot_device.is_none() {
                    fw.boot_device = Some(*entity);
                }
            }
            fw.phase = FirmwarePhase::DeviceReadyWait;
            println!("[Firmware] Waiting for devices to reach Ready");
        }
        FirmwarePhase::DeviceReadyWait => {
            let mut cpu_ready = false;
            let mut ram_ready = false;
            let mut mb_ready = false;

            for (entity, life) in life_query.iter() {
                if let Some(info) = registry.get(entity) {
                    let ready = life.phase == DevicePhase::Ready;
                    match info.kind {
                        DeviceKind::Cpu => cpu_ready |= ready,
                        DeviceKind::Ram => ram_ready |= ready,
                        DeviceKind::Motherboard => mb_ready |= ready,
                        _ => {}
                    }
                }
            }

            if cpu_ready && ram_ready && mb_ready {
                fw.phase = FirmwarePhase::Ready;
                println!("[Firmware] Machine READY for guest software");
            }
        }
        FirmwarePhase::Ready | FirmwarePhase::Off | FirmwarePhase::Halted => {}
    }
}
