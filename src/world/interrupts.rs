//! Interrupt system.
//!
//! Hardware raises interrupts into the world; the interrupt system is the
//! authority for pending vectors. Future software observes them here.

use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(Resource, Default)]
pub struct InterruptSystem {
    pub pending: VecDeque<PendingInterrupt>,
    pub masked: HashMap<u8, bool>,
}

#[derive(Clone, Debug)]
pub struct PendingInterrupt {
    pub vector: u8,
    pub source: Entity,
}

impl InterruptSystem {
    pub fn raise(&mut self, vector: u8, source: Entity) {
        if !*self.masked.get(&vector).unwrap_or(&false) {
            self.pending.push_back(PendingInterrupt { vector, source });
        }
    }

    pub fn acknowledge(&mut self) -> Option<PendingInterrupt> {
        self.pending.pop_front()
    }

    pub fn mask(&mut self, vector: u8) {
        self.masked.insert(vector, true);
    }

    pub fn unmask(&mut self, vector: u8) {
        self.masked.insert(vector, false);
    }
}

#[derive(Event, Clone, Debug)]
pub struct InterruptEvent {
    pub vector: u8,
    pub source: Entity,
}

/// Component: this device can raise interrupts.
#[derive(Component, Default)]
pub struct InterruptSource {
    pub default_vector: u8,
}

pub fn process_interrupts(
    mut system: ResMut<InterruptSystem>,
    mut events: EventWriter<InterruptEvent>,
) {
    // Emit events for any newly pending interrupts (simple model).
    for pending in system.pending.iter() {
        events.send(InterruptEvent {
            vector: pending.vector,
            source: pending.source,
        });
    }
}
