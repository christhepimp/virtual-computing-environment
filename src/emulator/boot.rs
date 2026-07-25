//! Minimal Linux boot experiment.
//!
//! Physics World → Virtual Hardware → Interfaces → Integration Layer → QEMU → Linux
//! QEMU is execution-only. Authority stays in world systems.

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
    mut memory: ResMut<MemoryMapSystem>,
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
            if let Some(kernel) = asset_path("vmlinuz").or_else(|| asset_path("bzImage")) {
                config.kernel_path = Some(kernel.clone());
                boot.kernel_loaded = true;
                println!("[Boot] Kernel found: {kernel}");
            } else {
                println!("[Boot] No kernel image in assets/boot/ (vmlinuz or bzImage)");
            }

            if let Some(initrd) =
                asset_path("initrd.img").or_else(|| asset_path("initramfs.cpio"))
            {
                config.initrd_path = Some(initrd.clone());
                boot.initrd_loaded = true;
                println!("[Boot] Initrd found: {initrd}");
            } else {
                println!("[Boot] No initrd found (optional)");
            }

            if let Some(disk) = asset_path("rootfs.img") {
                config.disk_path = Some(disk.clone());
                println!("[Boot] Host disk image: {disk}");
            }

            prepare_virtual_disk(&mut storage, &registry, &mut boot);
            seed_ram_boot_marker(&mut memory);

            boot.phase = BootPhase::ConfiguringGuest;
        }
        BootPhase::ConfiguringGuest => {
            println!(
                "[Boot] Guest config: arch={} machine={} ram={}MiB",
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

            if boot.kernel_loaded && integ.transport.state == QemuTransportState::Running {
                boot.last_status =
                    "QEMU running with kernel — watch serial for Linux boot".into();
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
        let mut sector = vec![0u8; 512];
        sector[0..4].copy_from_slice(b"VCE\x01");
        sector[510] = 0x55;
        sector[511] = 0xAA;
        if storage.write_sectors(entity, 0, &sector) {
            boot.disk_ready = true;
            println!(
                "[Boot][Storage] Virtual disk ready entity={entity:?} LBA0 signature written"
            );
        } else {
            println!("[Boot][Storage] Failed to write boot signature");
        }
    } else {
        println!("[Boot][Storage] No storage device registered");
    }
}

fn seed_ram_boot_marker(memory: &mut MemoryMapSystem) {
    let marker = b"VCE-RAM\0";
    let ok = memory.write_ram(0x0010_0000, marker);
    println!("[Boot][Memory] Seed RAM marker @0x00100000 ok={ok}");
    if let Some(data) = memory.read_ram(0x0010_0000, 8) {
        println!("[Boot][Memory] Readback {:02x?}", data);
    }
}

fn exercise_real_paths(
    integ: &mut EmulatorIntegration,
    memory: &MemoryMapSystem,
    storage: &StorageSystem,
    interrupts: &mut InterruptSystem,
    registry: &DeviceRegistry,
) {
    let addr = 0x0010_0000;
    integ.transport.enqueue_request(GuestRequest::MemoryWrite {
        addr,
        data: b"VCE-BOOT\0".to_vec(),
    });
    integ.transport.enqueue_request(GuestRequest::MemoryRead { addr, len: 8 });

    if let Some(&entity) = registry.devices_of_kind(DeviceKind::Storage).first() {
        integ.transport.enqueue_request(GuestRequest::StorageRead {
            entity_bits: entity.to_bits(),
            lba: 0,
            count: 1,
        });
        let _ = storage;
    }

    if let Some(&kbd) = registry.devices_of_kind(DeviceKind::Keyboard).first() {
        interrupts.raise(1, kbd);
        println!("[Boot][IRQ] Keyboard raised IRQ 1 via InterruptSystem");
    }

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
