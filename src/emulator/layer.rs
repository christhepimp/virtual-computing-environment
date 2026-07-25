//! Emulator Integration Layer.
//!
//! Sits between world systems and QEMU. Reads machine state from the world
//! (power, clock, memory map, interrupts, storage, buses) and presents a
//! coherent view to the emulator transport. Writes from the emulator side
//! are applied only through the same world interfaces an OS would use
//! (memory map, bus transactions, interrupt acknowledge, block I/O).
//!
//! This layer does not replace or bypass DeviceRegistry, PowerSystem,
//! MemoryMapSystem, or any other world system.

use bevy::prelude::*;

use crate::world::firmware::{Firmware, FirmwarePhase};
use crate::world::power::PowerSystem;
use crate::world::signals::{SignalId, SignalSystem};

use super::adapters::{
    ClockAdapter, DisplayAdapter, InputAdapter, InterruptAdapter, MemoryAdapter, PowerAdapter,
    StorageAdapter,
};
use super::config::MinimalMachineConfig;
use super::qemu::{QemuTransport, QemuTransportState};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EmulatorState {
    #[default]
    Stopped,
    /// Firmware is Ready; integration may start the guest.
    Arming,
    Running,
    Paused,
    Halted,
    Error,
}

#[derive(Event, Clone, Debug)]
pub enum EmulatorEvent {
    Started,
    Stopped,
    Paused,
    Resumed,
    Crashed(String),
    /// Guest attempted access; logged for boot experiments.
    GuestMemoryAccess { addr: u64, write: bool, len: usize },
    GuestInterrupt { vector: u8 },
}

/// Central integration object. Holds adapters and transport status.
/// Not a hardware owner — only a translation surface.
#[derive(Resource)]
pub struct EmulatorIntegration {
    pub state: EmulatorState,
    pub power: PowerAdapter,
    pub clock: ClockAdapter,
    pub memory: MemoryAdapter,
    pub interrupts: InterruptAdapter,
    pub storage: StorageAdapter,
    pub input: InputAdapter,
    pub display: DisplayAdapter,
    pub transport: QemuTransport,
    pub last_error: Option<String>,
}

impl Default for EmulatorIntegration {
    fn default() -> Self {
        Self {
            state: EmulatorState::Stopped,
            power: PowerAdapter::default(),
            clock: ClockAdapter::default(),
            memory: MemoryAdapter::default(),
            interrupts: InterruptAdapter::default(),
            storage: StorageAdapter::default(),
            input: InputAdapter::default(),
            display: DisplayAdapter::default(),
            transport: QemuTransport::default(),
            last_error: None,
        }
    }
}

/// Drive integration lifecycle from firmware + power only.
pub fn integration_lifecycle(
    firmware: Res<Firmware>,
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    config: Res<MinimalMachineConfig>,
    mut integ: ResMut<EmulatorIntegration>,
    mut events: EventWriter<EmulatorEvent>,
) {
    let machine_ready = firmware.phase == FirmwarePhase::Ready
        && power.main_power
        && signals.is_asserted(SignalId::PowerGood);

    match integ.state {
        EmulatorState::Stopped | EmulatorState::Halted => {
            if machine_ready {
                integ.state = EmulatorState::Arming;
                println!("[EmulatorLayer] Machine ready — arming guest integration");
            }
        }
        EmulatorState::Arming => {
            if !machine_ready {
                integ.state = EmulatorState::Stopped;
                return;
            }
            // Attempt to start transport (may be dry-run if QEMU not installed).
            match integ.transport.start(&config) {
                Ok(()) => {
                    integ.state = EmulatorState::Running;
                    events.send(EmulatorEvent::Started);
                    println!("[EmulatorLayer] Guest transport started ({})", integ.transport.mode_name());
                }
                Err(e) => {
                    integ.last_error = Some(e.clone());
                    // Stay armed in dry-run so adapters still sync for experiments.
                    integ.state = EmulatorState::Running;
                    integ.transport.state = QemuTransportState::DryRun;
                    events.send(EmulatorEvent::Started);
                    println!("[EmulatorLayer] QEMU unavailable ({e}) — running in dry-run adapter mode");
                }
            }
        }
        EmulatorState::Running | EmulatorState::Paused => {
            if !machine_ready {
                integ.transport.stop();
                integ.state = EmulatorState::Halted;
                events.send(EmulatorEvent::Stopped);
                println!("[EmulatorLayer] Machine not ready — guest halted");
            }
        }
        EmulatorState::Error => {}
    }
}

pub fn sync_power_adapter(
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    mut integ: ResMut<EmulatorIntegration>,
) {
    if integ.state != EmulatorState::Running && integ.state != EmulatorState::Paused {
        return;
    }
    integ.power.sync_from_world(&power, &signals);
}

pub fn sync_clock_adapter(
    clock: Res<crate::world::clock::ClockSystem>,
    mut integ: ResMut<EmulatorIntegration>,
) {
    if integ.state != EmulatorState::Running && integ.state != EmulatorState::Paused {
        return;
    }
    integ.clock.sync_from_world(&clock);
}

pub fn sync_memory_adapter(
    memory: Res<crate::world::memory::MemoryMapSystem>,
    mut integ: ResMut<EmulatorIntegration>,
) {
    if integ.state != EmulatorState::Running && integ.state != EmulatorState::Paused {
        return;
    }
    integ.memory.sync_map_from_world(&memory);
}

pub fn sync_interrupt_adapter(
    interrupts: Res<crate::world::interrupts::InterruptSystem>,
    mut integ: ResMut<EmulatorIntegration>,
    mut events: EventWriter<EmulatorEvent>,
) {
    if integ.state != EmulatorState::Running && integ.state != EmulatorState::Paused {
        return;
    }
    for vector in integ.interrupts.pull_pending_from_world(&interrupts) {
        events.send(EmulatorEvent::GuestInterrupt { vector });
        integ.transport.notify_interrupt(vector);
    }
}

pub fn sync_storage_adapter(
    storage: Res<crate::world::storage::StorageSystem>,
    mut integ: ResMut<EmulatorIntegration>,
) {
    if integ.state != EmulatorState::Running && integ.state != EmulatorState::Paused {
        return;
    }
    integ.storage.sync_from_world(&storage);
}

pub fn sync_input_adapter(mut integ: ResMut<EmulatorIntegration>) {
    if integ.state != EmulatorState::Running {
        return;
    }
    // Input adapter drains queues toward transport when present.
    integ.input.flush_to_transport(&mut integ.transport);
}

pub fn pump_qemu_transport(
    mut integ: ResMut<EmulatorIntegration>,
    mut memory: ResMut<crate::world::memory::MemoryMapSystem>,
    mut storage: ResMut<crate::world::storage::StorageSystem>,
    mut interrupts: ResMut<crate::world::interrupts::InterruptSystem>,
    mut events: EventWriter<EmulatorEvent>,
) {
    if integ.state != EmulatorState::Running {
        return;
    }

    // Process guest-side requests coming back through the transport.
    let requests = integ.transport.poll_requests();
    for req in requests {
        match req {
            super::qemu::GuestRequest::MemoryRead { addr, len } => {
                events.send(EmulatorEvent::GuestMemoryAccess {
                    addr,
                    write: false,
                    len,
                });
                let data = integ.memory.read_through_world(&memory, addr, len);
                integ.transport.complete_memory_read(addr, data);
            }
            super::qemu::GuestRequest::MemoryWrite { addr, data } => {
                events.send(EmulatorEvent::GuestMemoryAccess {
                    addr,
                    write: true,
                    len: data.len(),
                });
                let _ = integ.memory.write_through_world(&mut memory, addr, &data);
            }
            super::qemu::GuestRequest::StorageRead { entity_bits, lba, count } => {
                let entity = Entity::from_bits(entity_bits);
                let data = storage.read_sectors(entity, lba, count);
                integ.transport.complete_storage_read(entity_bits, data);
            }
            super::qemu::GuestRequest::StorageWrite {
                entity_bits,
                lba,
                data,
            } => {
                let entity = Entity::from_bits(entity_bits);
                let _ = storage.write_sectors(entity, lba, &data);
            }
            super::qemu::GuestRequest::InterruptAck { vector } => {
                // Acknowledge by draining matching pending entries.
                let _ = vector;
                let _ = interrupts.acknowledge();
            }
        }
    }
}
