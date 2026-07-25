//! Motherboard bus arbitration — only active when motherboard lifecycle is Ready.

use bevy::prelude::*;

use super::buses::{BusId, BusSystem, BusTransactionCompleted, TransactionKind};
use super::devices::{DeviceKind, DeviceRegistry, Registered};
use super::lifecycle::{DeviceLifecycle, DevicePhase};
use super::memory::MemoryMapSystem;
use super::power::PowerSystem;
use crate::hardware::Motherboard;

pub fn motherboard_bus_router(
    power: Res<PowerSystem>,
    registry: Res<DeviceRegistry>,
    mut buses: ResMut<BusSystem>,
    mut memory: ResMut<MemoryMapSystem>,
    mut completed_events: EventWriter<BusTransactionCompleted>,
    mb_query: Query<(Entity, &DeviceLifecycle), (With<Motherboard>, With<Registered>)>,
) {
    if !power.main_power {
        return;
    }

    let mb_ready = mb_query.iter().any(|(e, life)| {
        power.is_powered(e) && life.phase == DevicePhase::Ready
    });
    if !mb_ready {
        return;
    }

    // Buses are active only while motherboard is Ready.
    let _ = registry.devices_of_kind(DeviceKind::Motherboard);

    for bus_id in [BusId::System, BusId::Memory, BusId::Io] {
        while let Some(mut txn) = buses.take_pending(bus_id) {
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

            // No handler yet — complete with error so requesters do not hang.
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
