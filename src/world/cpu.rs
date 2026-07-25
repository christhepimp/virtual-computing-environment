//! CPU behavior at the architectural level.
//!
//! The CPU issues bus transactions to fetch/store data. It does not reach
//! into RAM or devices directly. Future guest software will drive richer
//! execution; this module provides a minimal, interface-correct core.

use bevy::prelude::*;

use super::buses::{BusId, BusSystem, TransactionKind};
use super::devices::{DeviceKind, DeviceRegistry, Registered};
use super::firmware::{Firmware, FirmwarePhase};
use super::power::PowerSystem;
use crate::hardware::Cpu;

#[derive(Component, Default)]
pub struct CpuCore {
    pub program_counter: u64,
    pub halted: bool,
    /// Simple register file placeholder (for future instruction work).
    pub regs: [u64; 16],
    pub pending_txn: Option<u64>,
}

/// When firmware is Ready and power is on, the CPU is eligible to run.
/// This tick only demonstrates issuing a memory read through the bus.
pub fn cpu_tick(
    power: Res<PowerSystem>,
    firmware: Res<Firmware>,
    registry: Res<DeviceRegistry>,
    mut buses: ResMut<BusSystem>,
    mut query: Query<(Entity, &mut CpuCore), (With<Cpu>, With<Registered>)>,
) {
    if !power.main_power || firmware.phase != FirmwarePhase::Ready {
        return;
    }

    for (entity, mut core) in query.iter_mut() {
        if !power.is_powered(entity) || core.halted {
            continue;
        }

        // If a previous transaction completed, clear pending.
        if let Some(txn_id) = core.pending_txn {
            if let Some(txn) = buses.take_completed_for(BusId::Memory, entity) {
                if txn.id == txn_id && !txn.error {
                    // Could load into a register — placeholder.
                    if let Some(data) = txn.response {
                        if data.len() >= 8 {
                            let mut buf = [0u8; 8];
                            buf.copy_from_slice(&data[..8]);
                            core.regs[0] = u64::from_le_bytes(buf);
                        }
                    }
                }
                core.pending_txn = None;
                core.program_counter = core.program_counter.wrapping_add(4);
            }
            continue;
        }

        // Issue a read from the current PC (demonstrates bus usage).
        // Real instruction fetch would use the same path.
        let id = buses.issue(
            BusId::Memory,
            TransactionKind::Read,
            core.program_counter,
            vec![0; 8], // request 8 bytes
            entity,
        );
        core.pending_txn = Some(id);

        // Only one CPU for now; avoid flooding the bus every frame in demos.
        let _ = registry.devices_of_kind(DeviceKind::Cpu);
        break;
    }
}
