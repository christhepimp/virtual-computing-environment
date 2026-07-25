//! Hardware lifecycle and per-device state machines.
//!
//! Devices progress through realistic phases driven only by power, clock,
//! signals, buses, and firmware — never by direct cross-calls between
//! component types. Loss of power or disconnection forces devices back
//! through failure / offline states via the same world systems.

use bevy::prelude::*;

/// Shared lifecycle phases for any powered hardware device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DevicePhase {
    #[default]
    Offline,
    PowerApplied,
    ResetHold,
    Initializing,
    Online,
    Ready,
    Error,
    Failed,
}

/// Authoritative lifecycle state living on the entity but advanced only
/// by world systems (power/clock/signals/firmware).
#[derive(Component, Debug)]
pub struct DeviceLifecycle {
    pub phase: DevicePhase,
    pub init_ticks: u32,
    pub required_init_ticks: u32,
    pub error_reason: Option<&'static str>,
}

impl DeviceLifecycle {
    pub fn new(required_init_ticks: u32) -> Self {
        Self {
            phase: DevicePhase::Offline,
            init_ticks: 0,
            required_init_ticks,
            error_reason: None,
        }
    }

    pub fn force_offline(&mut self) {
        self.phase = DevicePhase::Offline;
        self.init_ticks = 0;
        self.error_reason = None;
    }

    pub fn fail(&mut self, reason: &'static str) {
        self.phase = DevicePhase::Failed;
        self.error_reason = Some(reason);
    }
}

/// Power-button edge detector component.
#[derive(Component, Default)]
pub struct PowerButtonState {
    pub pressed: bool,
    pub last_pressed: bool,
}

/// Power-supply rail state.
#[derive(Component, Debug, Default)]
pub struct PowerSupplyState {
    pub spinning_up: bool,
    pub rails_stable: bool,
    pub spin_ticks: u32,
    pub required_spin_ticks: u32,
}

impl PowerSupplyState {
    pub fn new() -> Self {
        Self {
            spinning_up: false,
            rails_stable: false,
            spin_ticks: 0,
            required_spin_ticks: 30, // foundation timing units
        }
    }
}
