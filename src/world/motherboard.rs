//! Motherboard coordination.
//!
//! The motherboard routes bus traffic between CPU, memory, and devices.
//! It does not implement device logic itself — it arbitrates and forwards.

use bevy::prelude::*;

use super::buses::{BusId, BusSystem, BusTransactionCompleted, TransactionKind};
use super::devices::{DeviceKind, DeviceRegistry, Registered};
use super::memory::MemoryMapSystem;
use super::power::PowerSystem;
use crate::hardware::Motherboard;

/// Process pending system/memory/io transactions while power is on.
/// Memory reads/writes are satisfied from MemoryMapSystem when possible.
/// Other addresses remain pending for device controllers to claim.
pub fn motherboard_bus_router(
    power: Res<PowerSystem>,
    registry: Res<DeviceRegistry>,
    mut buses: ResMut<BusSystem>,
    mut memory: ResMut<MemoryMapSystem>,
    mut completed_events: EventWriter<BusTransactionCompleted>,
    mb_query: Query<Entity, (With<Motherboard>, With<Registered>)>,
) {
    if !power.main_power {
        return;
    }
    if mb_query.iter().next().is_none() {
        return;
    }
    // Motherboard present and powered — route traffic.
    let mb_entities = registry.devices_of_kind(DeviceKind::Motherboard);
    if mb_entities.is_empty() {
        return;
    }
    let mb = mb_entities[0];
    if !power.is_powered(mb) {
        return;
    }

    for bus_id in [BusId::System, BusId::Memory, BusId::Io] {
        while let Some(mut txn) = buses.take_pending(bus_id) {
            // Try to satisfy from RAM first.
            match txn.kind {
                TransactionKind::Read => {
                    if let Some(data) = memory.read_ram(txn.address, txn.data.len().max(1)) {
                        txn.response = Some(data);
                        txn.completed = true;
                        txn.error = false;
                        completed_events.send(BusTransactionCompleted {
                            transaction: txn.clone(),
                        });
                        buses.complete(txn);
                        continue;
                    }
                }
                TransactionKind::Write => {
                    if memory.write_ram(txn.address, &txn.data) {
                        txn.response = Some(vec![]);
                        txn.completed = true;
                        txn.error = false;
                        completed_events.send(BusTransactionCompleted {
                            transaction: txn.clone(),
                        });
                        buses.complete(txn);
                        continue;
                    }
                }
            }

            // Not RAM — leave on a side queue for device controllers by re-completing
            // as incomplete only if we can't handle; for foundation, mark error if
            // no one claims. Device controllers will get a future pass.
            // For now, put back as completed with error so requesters don't hang.
            txn.completed = true;
            txn.error = true;
            txn.response = None;
            completed_events.send(BusTransactionCompleted {
                transaction: txn.clone(),
            });
            buses.complete(txn);
        }
    }
}
