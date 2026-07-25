//! CPU core — runs only when lifecycle is Ready and clock is enabled.

use bevy::prelude::*;

use super::buses::{BusId, BusSystem, TransactionKind};
use super::devices::Registered;
use super::firmware::{Firmware, FirmwarePhase};
use super::lifecycle::{DeviceLifecycle, DevicePhase};
use super::power::PowerSystem;
use crate::hardware::Cpu;

#[derive(Component, Default)]
pub struct CpuCore {
    pub program_counter: u64,
    pub halted: bool,
    pub regs: [u64; 16],
    pub pending_txn: Option<u64>,
}

pub fn cpu_tick(
    power: Res<PowerSystem>,
    firmware: Res<Firmware>,
    mut buses: ResMut<BusSystem>,
    mut query: Query<
        (Entity, &mut CpuCore, &DeviceLifecycle),
        (With<Cpu>, With<Registered>),
    >,
) {
    if !power.main_power || firmware.phase != FirmwarePhase::Ready {
        return;
    }

    for (entity, mut core, life) in query.iter_mut() {
        if !power.is_powered(entity) || life.phase != DevicePhase::Ready || core.halted {
            continue;
        }

        if let Some(txn_id) = core.pending_txn {
            if let Some(txn) = buses.take_completed_for(BusId::Memory, entity) {
                if txn.id == txn_id && !txn.error {
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

        // Instruction-fetch style memory read through the bus only.
        let id = buses.issue(
            BusId::Memory,
            TransactionKind::Read,
            core.program_counter.max(0x0010_0000), // stay inside RAM window
            vec![0; 8],
            entity,
        );
        core.pending_txn = Some(id);
        break;
    }
}
