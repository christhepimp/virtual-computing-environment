//! Bus adapter — optional path for MMIO/PIO style accesses via BusSystem.

use bevy::prelude::Entity;

use crate::world::buses::{BusId, BusSystem, TransactionKind};

#[derive(Default, Debug)]
pub struct BusAdapter;

impl BusAdapter {
    pub fn issue_read(
        buses: &mut BusSystem,
        bus: BusId,
        address: u64,
        len: usize,
        requester: Entity,
    ) -> u64 {
        buses.issue(bus, TransactionKind::Read, address, vec![0; len], requester)
    }

    pub fn issue_write(
        buses: &mut BusSystem,
        bus: BusId,
        address: u64,
        data: Vec<u8>,
        requester: Entity,
    ) -> u64 {
        buses.issue(bus, TransactionKind::Write, address, data, requester)
    }
}
