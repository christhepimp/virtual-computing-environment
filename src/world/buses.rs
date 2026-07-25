//! Virtual bus system with transaction-oriented communication.
//!
//! Devices do not call each other directly. They issue bus transactions.
//! The motherboard (and bus system) arbitrate and route them.
//! This is the primary path an OS would use to talk to hardware.

use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BusId {
    System,
    Memory,
    Io,
    Custom(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionKind {
    Read,
    Write,
}

#[derive(Clone, Debug)]
pub struct BusTransaction {
    pub id: u64,
    pub bus: BusId,
    pub kind: TransactionKind,
    pub address: u64,
    pub data: Vec<u8>,
    pub requester: Entity,
    /// Filled by the responding device / memory system.
    pub response: Option<Vec<u8>>,
    pub completed: bool,
    pub error: bool,
}

#[derive(Clone, Debug, Default)]
pub struct BusState {
    pub participants: Vec<Entity>,
    pub pending: VecDeque<BusTransaction>,
    pub completed: VecDeque<BusTransaction>,
    next_txn_id: u64,
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

    /// Issue a transaction onto a bus. Returns transaction id.
    pub fn issue(
        &mut self,
        bus: BusId,
        kind: TransactionKind,
        address: u64,
        data: Vec<u8>,
        requester: Entity,
    ) -> u64 {
        let state = self.ensure_bus(bus);
        let id = state.next_txn_id;
        state.next_txn_id = state.next_txn_id.wrapping_add(1);
        state.pending.push_back(BusTransaction {
            id,
            bus,
            kind,
            address,
            data,
            requester,
            response: None,
            completed: false,
            error: false,
        });
        id
    }

    pub fn take_pending(&mut self, bus: BusId) -> Option<BusTransaction> {
        self.buses.get_mut(&bus)?.pending.pop_front()
    }

    pub fn complete(&mut self, mut txn: BusTransaction) {
        txn.completed = true;
        if let Some(state) = self.buses.get_mut(&txn.bus) {
            state.completed.push_back(txn);
        }
    }

    pub fn take_completed_for(&mut self, bus: BusId, requester: Entity) -> Option<BusTransaction> {
        let state = self.buses.get_mut(&bus)?;
        if let Some(pos) = state
            .completed
            .iter()
            .position(|t| t.requester == requester)
        {
            return state.completed.remove(pos);
        }
        None
    }
}

/// Component: this entity is attached to one or more buses.
#[derive(Component, Default)]
pub struct BusAttachment {
    pub buses: Vec<BusId>,
}

/// Event when a transaction completes (for observers / debugging).
#[derive(Event, Clone, Debug)]
pub struct BusTransactionCompleted {
    pub transaction: BusTransaction,
}
