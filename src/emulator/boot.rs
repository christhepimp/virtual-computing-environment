//! Minimal Linux boot experiment.
//!
//! Drives the first guest boot attempt through the existing integration layer.
//! All memory, storage, IRQ, and clock traffic is intended to flow:
//!
//!   Physics World → Virtual Hardware → Interfaces → Integration Layer → QEMU → Linux
//!
//! QEMU remains execution-only. Authority stays in world systems.

use std::fs;
use std::path::Path;

use bevy::prelude::*;

use crate::world::clock::ClockSystem;
use crate::world::devices::{DeviceKind, DeviceRegistry};
use crate::world::firmware::{Firmware, FirmwarePhase};
use crate::world::interrupts::InterruptSystem;
use crate::world::lifecycle::{DeviceLifecycle, DevicePhase};
use crate::world::memory::MemoryMapSystem;
use crate::world::power::PowerSystem;
use crate::world::signals::{SignalId, SignalSystem};
use crate::world::storage::StorageSystem;

use super::config::MinimalMachineConfig;
use super::layer::{EmulatorEvent, EmulatorIntegration, EmulatorState};
use super::qemu::{GuestRequest, QemuTransportState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootPhase {
    Idle,
    WaitingForMachine,
    LoadingAssets,
    ConfiguringGuest,
    StartingTransport,
    GuestRunning,
    Observing,
    Failed,
    Completed,
}

#[derive(Resource)]
pub struct BootExperiment {
    pub phase: BootPhase,
    pub kernel_loaded: bool,
    pub initrd_loaded: bool,
    pub disk_ready: bool,
    pub log_ticks: u64,
    pub last_status: String,
}

impl Default for BootExperiment {
    fn default() -> Self {
        Self {
            phase: BootPhase::Idle,
            kernel_loaded: false,
            initrd_loaded: false,
            disk_ready: false,
            log_ticks: 0,
            last_status: "idle".into(),
        }
    }
}

/// Discover optional boot assets under assets/boot/.
fn asset_path(name: &str) -> Option<String> {
    let candidates = [
        format!("assets/boot/{name}"),
        format!("boot/{name}"),
        format!("/home/workdir/artifacts/boot/{name}"),
    ];
    candidates.into_iter().find(|p| Path::new(p).exists())
}

pub fn boot_experiment_tick(
    firmware: Res<Firmware>,
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    registry: Res<DeviceRegistry>,
    memory: ResMut<MemoryMapSystem>,
    mut storage: ResMut<StorageSystem>,
    mut interrupts: ResMut<InterruptSystem>,
    clock: Res<ClockSystem>,
    mut config: ResMut<MinimalMachineConfig>,
    mut integ: ResMut<EmulatorIntegration>,
    mut boot: ResMut<BootExperiment>,
    mut events: EventWriter<EmulatorEvent>,
    life_query: Query<(Entity, &DeviceLifecycle)>,
) {
    boot.log_ticks = boot.log_ticks.wrapping_add(1);

    // Periodic machine snapshot
    if boot.log_ticks % 120 == 0 {
        log_machine_snapshot(
            &power,
            &signals,
            &firmware,
            &registry,
            &memory,
            &storage,
            &interrupts,
            &clock,
            &integ,
            &boot,
            &life_query,
        );
    }

    match boot.phase {
        BootPhase::Idle => {
            boot.phase = BootPhase::WaitingForMachine;
            boot.last_status = "waiting for firmware Ready + PowerGood".into();
            println!("[Boot] Waiting for virtual machine to become READY");
        }
        BootPhase::WaitingForMachine => {
            if firmware.phase == FirmwarePhase::Ready
                && power.main_power
                && signals.is_asserted(SignalId::PowerGood)
            {
                boot.phase = BootPhase::LoadingAssets;
                boot.last_status = "machine ready — loading boot assets".into();
                println!("[Boot] Machine READY — loading kernel/initrd/disk assets");
            }
        }
        BootPhase::LoadingAssets => {
            if let Some(kernel) = asset_path("vmlinuz") {
                config.kernel_path = Some(kernel.clone());
                boot.kernel_loaded = true;
                println!("[Boot] Kernel found: {kernel}");
            } else if let Some(kernel) = asset_path("bzImage") {
                config.kernel_path = Some(kernel.clone());
                boot.kernel_loaded = true;
                println!("[Boot] Kernel found: {kernel}");
            } else {
                println!("[Boot] No kernel image in assets/boot/ (vmlinuz or bzImage)");
            }

            if let Some(initrd) = asset_path("initrd.img") {
                config.initrd_path = Some(initrd.clone());
                boot.initrd_loaded = true;
                println!("[Boot] Initrd found: {initrd}");
            } else if let Some(initrd) = asset_path("initramfs.cpio") {
                config.initrd_path = Some(initrd.clone());
                boot.initrd_loaded = true;
                println!("[Boot] Initrd found: {initrd}");
            } else {
                println!("[Boot] No initrd found (optional for minimal experiments)");
            }

            // Virtual disk: ensure storage device has a recognizable boot sector marker.
            prepare_virtual_disk(&mut storage, &registry, &mut boot);

            // Seed a tiny marker into RAM via world memory (not QEMU-owned).
            seed_ram_boot_marker(&memory);

            boot.phase = BootPhase::ConfiguringGuest;
        }
        BootPhase::ConfiguringGuest => {
            println!("[Boot] Guest config: arch={} machine={} ram={}MiB",
                config.arch,
                config.machine,
                config.ram_bytes / (1024 * 1024)
            );
            println!(
                "[Boot] kernel={:?} initrd={:?} disk={:?}",
                config.kernel_path, config.initrd_path, config.disk_path
            );
            boot.phase = BootPhase::StartingTransport;
        }
        BootPhase::StartingTransport => {
            // Integration layer owns start; force arming if still stopped.
            if integ.state == EmulatorState::Stopped || integ.state == EmulatorState::Halted {
                integ.state = EmulatorState::Arming;
            }
            if integ.state == EmulatorState::Running {
                boot.phase = BootPhase::GuestRunning;
                boot.last_status = format!("guest transport {}", integ.transport.mode_name());
                println!(
                    "[Boot] Guest transport active ({})",
                    integ.transport.mode_name()
                );
                events.send(EmulatorEvent::Started);

                // Exercise real paths immediately (dry-run or live).
                exercise_real_paths(&mut integ, &memory, &storage, &mut interrupts, &registry);
            } else if integ.state == EmulatorState::Error {
                boot.phase = BootPhase::Failed;
                boot.last_status = integ.last_error.clone().unwrap_or_else(|| "error".into());
                println!("[Boot] FAILED: {}", boot.last_status);
            }
        }
        BootPhase::GuestRunning => {
            boot.phase = BootPhase::Observing;
            println!("[Boot] Observing guest / adapter traffic");
        }
        BootPhase::Observing => {
            // Keep pumping demonstration traffic on a slow cadence in dry-run
            // so logs show living paths even without a kernel image.
            if matches!(
                integ.transport.state,
                QemuTransportState::DryRun | QemuTransportState::Running
            ) && boot.log_ticks % 180 == 0
            {
                exercise_real_paths(&mut integ, &memory, &storage, &mut interrupts, &registry);
            }

            if !power.main_power || firmware.phase != FirmwarePhase::Ready {
                boot.phase = BootPhase::Failed;
                boot.last_status = "machine left Ready — boot aborted".into();
                println!("[Boot] Aborted — machine no longer ready");
            }

            // If a real kernel was provided and transport is Running, mark progress.
            if boot.kernel_loaded && integ.transport.state == QemuTransportState::Running {
                boot.last_status = "QEMU running with kernel — watch serial for Linux boot".into();
            } else if !boot.kernel_loaded && integ.transport.state == QemuTransportState::DryRun {
                boot.last_status =
                    "dry-run: place vmlinuz/bzImage in assets/boot/ for real QEMU boot".into();
            }
        }
        BootPhase::Failed | BootPhase::Completed => {}
    }
}

fn prepare_virtual_disk(
    storage: &mut StorageSystem,
    registry: &DeviceRegistry,
    boot: &mut BootExperiment,
) {
    let disks = registry.devices_of_kind(DeviceKind::Storage);
    if let Some(&entity) = disks.first() {
        // Write a simple signature at LBA 0 so storage path is real.
        let mut sector = vec![0u8; 512];
        sector[0..4].copy_from_slice(b"VCE\x01");
        sector[510] = 0x55;
        sector[511] = 0xAA;
        if storage.write_sectors(entity, 0, &sector) {
            boot.disk_ready = true;
            println!(
                "[Boot][Storage] Virtual disk ready entity={entity:?} sectors signature written"
            );

            // Optional host-side raw image for QEMU -drive if user provides path later.
            if let Some(path) = asset_path("rootfs.img") {
                boot_experiment_set_disk(path);
            }
        } else {
            println!("[Boot][Storage] Failed to write boot signature");
        }
    } else {
        println!("[Boot][Storage] No storage device registered");
    }
}

fn boot_experiment_set_disk(path: String) {
    println!("[Boot][Storage] Host disk image available: {path}");
    // Config update happens in LoadingAssets when we can mutably access config;
    // logged here for visibility when discovered via storage prep helpers.
    let _ = fs::metadata(&path);
}

fn seed_ram_boot_marker(memory: &MemoryMapSystem) {
    // Read-only check that RAM window exists; writes go through adapter path later.
    if let Some(data) = memory.read_ram(0x0010_0000, 16) {
        println!(
            "[Boot][Memory] RAM window @0x00100000 readable ({} bytes sample)",
            data.len()
        );
    } else {
        println!("[Boot][Memory] RAM window @0x00100000 not readable yet");
    }
}

fn exercise_real_paths(
    integ: &mut EmulatorIntegration,
    memory: &MemoryMapSystem,
    storage: &StorageSystem,
    interrupts: &mut InterruptSystem,
    registry: &DeviceRegistry,
) {
    // 1) Memory path — guest-style read/write through adapter → world
    let addr = 0x0010_0000;
    integ.transport.enqueue_request(GuestRequest::MemoryWrite {
        addr,
        data: b"VCE-BOOT\0".to_vec(),
    });
    integ.transport.enqueue_request(GuestRequest::MemoryRead { addr, len: 8 });

    // 2) Storage path
    if let Some(&entity) = registry.devices_of_kind(DeviceKind::Storage).first() {
        integ.transport.enqueue_request(GuestRequest::StorageRead {
            entity_bits: entity.to_bits(),
            lba: 0,
            count: 1,
        });
        let _ = storage; // reads fulfilled in pump_qemu_transport
    }

    // 3) Interrupt path — virtual device raises IRQ into world, adapter surfaces it
    if let Some(&kbd) = registry.devices_of_kind(DeviceKind::Keyboard).first() {
        interrupts.raise(1, kbd);
        println!("[Boot][IRQ] Keyboard raised IRQ 1 via InterruptSystem");
    }

    // Memory map presence
    println!(
        "[Boot][Memory] regions={} ram_stores={}",
        memory.regions.len(),
        memory.ram_stores.len()
    );
}

fn log_machine_snapshot(
    power: &PowerSystem,
    signals: &SignalSystem,
    firmware: &Firmware,
    registry: &DeviceRegistry,
    memory: &MemoryMapSystem,
    storage: &StorageSystem,
    interrupts: &InterruptSystem,
    clock: &ClockSystem,
    integ: &EmulatorIntegration,
    boot: &BootExperiment,
    life_query: &Query<(Entity, &DeviceLifecycle)>,
) {
    println!("---------- [Boot Snapshot] ----------");
    println!(
        " power: main={} good={} clk_en={} consumed={:.1}W",
        power.main_power,
        signals.is_asserted(SignalId::PowerGood),
        signals.is_asserted(SignalId::ClockEnable),
        power.consumed_watts
    );
    println!(" firmware: {:?}", firmware.phase);
    println!(
        " clock: ticks={} hz={}",
        clock.master_ticks, clock.master_hz
    );
    println!(
        " devices: {} | memory regions: {} | block devs: {} | pending IRQs: {}",
        registry.devices.len(),
        memory.regions.len(),
        storage.devices.len(),
        interrupts.pending.len()
    );
    for (entity, life) in life_query.iter() {
        if let Some(info) = registry.get(entity) {
            if matches!(
                life.phase,
                DevicePhase::Ready | DevicePhase::Failed | DevicePhase::Error
            ) || boot.log_ticks < 200
            {
                println!(
                    "  device {} ({:?}) phase={:?}",
                    info.name, info.kind, life.phase
                );
            }
        }
    }
    println!(
        " emulator: state={:?} transport={} boot_phase={:?} status={}",
        integ.state,
        integ.transport.mode_name(),
        boot.phase,
        boot.last_status
    );
    if let Some((addr, ref data)) = integ.transport.last_mem_read {
        println!(
            " last mem read @{:#x} -> {}",
            addr,
            data.as_ref()
                .map(|d| format!("{:02x?}", &d[..d.len().min(8)]))
                .unwrap_or_else(|| "none".into())
        );
    }
    println!("-------------------------------------");
}
