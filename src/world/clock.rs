//! Clock system — global and per-device timing.
//!
//! Provides the master clock that hardware and future software can observe.
//! Hardware does not maintain its own independent time authority.

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct ClockSystem {
    /// Master ticks since power-on (or world start).
    pub master_ticks: u64,
    /// Hertz of the master clock (abstract for now).
    pub master_hz: u64,
    /// Optional per-device clock domains.
    pub device_clocks: HashMap<Entity, DeviceClock>,
}

#[derive(Clone, Debug, Default)]
pub struct DeviceClock {
    pub ticks: u64,
    pub hz: u64,
    pub enabled: bool,
}

impl ClockSystem {
    pub fn initialize(&mut self) {
        self.master_ticks = 0;
        self.master_hz = 100_000_000; // 100 MHz abstract baseline
        self.device_clocks.clear();
    }

    pub fn register_device(&mut self, entity: Entity, hz: u64) {
        self.device_clocks.insert(
            entity,
            DeviceClock {
                ticks: 0,
                hz,
                enabled: false,
            },
        );
    }

    pub fn unregister_device(&mut self, entity: Entity) {
        self.device_clocks.remove(&entity);
    }
}

/// Marker: this entity has a clock domain managed by the world.
#[derive(Component)]
pub struct ClockedDevice {
    pub hz: u64,
}

pub fn clock_tick(mut clock: ResMut<ClockSystem>, power: Res<super::power::PowerSystem>) {
    if !power.main_power {
        return;
    }

    clock.master_ticks = clock.master_ticks.wrapping_add(1);

    for (_entity, device_clock) in clock.device_clocks.iter_mut() {
        if device_clock.enabled {
            device_clock.ticks = device_clock.ticks.wrapping_add(1);
        }
    }
}
