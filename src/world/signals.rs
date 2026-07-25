//! Signal system — discrete signals between devices (power-good, reset, etc.).
//!
//! Signals are routed through the world; devices do not hold private signal state
//! as the authority.

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SignalId {
    Reset,
    PowerGood,
    ClockEnable,
    Custom(u32),
}

#[derive(Resource, Default)]
pub struct SignalSystem {
    /// Current level of each named signal (true = asserted).
    pub levels: HashMap<SignalId, bool>,
    /// Who is driving / observing (for future routing).
    pub drivers: HashMap<SignalId, Vec<Entity>>,
}

impl SignalSystem {
    pub fn assert(&mut self, id: SignalId) {
        self.levels.insert(id, true);
    }

    pub fn deassert(&mut self, id: SignalId) {
        self.levels.insert(id, false);
    }

    pub fn is_asserted(&self, id: SignalId) -> bool {
        *self.levels.get(&id).unwrap_or(&false)
    }
}

#[derive(Event, Clone, Debug)]
pub struct SignalEvent {
    pub signal: SignalId,
    pub asserted: bool,
    pub source: Option<Entity>,
}

pub fn process_signals(
    signals: Res<SignalSystem>,
    mut events: EventWriter<SignalEvent>,
) {
    // Placeholder for edge detection / broadcast in later stages.
    let _ = (signals, events);
}
