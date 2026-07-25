//! Virtual bus system.
//!
//! Buses are world-level communication channels. Hardware attaches to them;
//! the bus system owns the authoritative traffic state.

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BusId {
    System,
    Memory,
    Io,
    Custom(u32),
}

#[derive(Clone, Debug, Default)]
pub struct BusState {
    pub data: Vec<u8>,
    pub address: u64,
    pub participants: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct BusSystem {
    pub buses: HashMap<BusId, BusState>,
}

impl BusSystem {
    pub fn ensure_bus(&mut self, id: BusId) -> &mut BusState {
        self.buses.entry(id).or_default()
    }

    pub fn attach(&mut self, id: BusId, entity: Entity) {
        let bus = self.ensure_bus(id);
        if !bus.participants.contains(&entity) {
            bus.participants.push(entity);
        }
    }

    pub fn detach(&mut self, id: BusId, entity: Entity) {
        if let Some(bus) = self.buses.get_mut(&id) {
            bus.participants.retain(|&e| e != entity);
        }
    }

    pub fn write(&mut self, id: BusId, address: u64, data: &[u8]) {
        let bus = self.ensure_bus(id);
        bus.address = address;
        bus.data = data.to_vec();
    }

    pub fn read(&self, id: BusId) -> Option<&[u8]> {
        self.buses.get(&id).map(|b| b.data.as_slice())
    }
}

/// Component: this entity is attached to one or more buses.
#[derive(Component, Default)]
pub struct BusAttachment {
    pub buses: Vec<BusId>,
}
