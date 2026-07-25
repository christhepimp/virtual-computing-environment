//! Power system — world-level authority for power state.
//!
//! Hardware does not store its own authoritative power state.
//! It queries and reacts through this system.

use bevy::prelude::*;
use std::collections::HashMap;

/// Global power state managed by the world.
#[derive(Resource, Default)]
pub struct PowerSystem {
    /// Whether main power rail is live.
    pub main_power: bool,
    /// Per-device power state (entity → powered).
    pub device_power: HashMap<Entity, bool>,
    /// Available power budget (abstract units for future simulation).
    pub available_watts: f32,
    pub consumed_watts: f32,
}

impl PowerSystem {
    pub fn initialize(&mut self) {
        self.main_power = false;
        self.available_watts = 500.0;
        self.consumed_watts = 0.0;
        self.device_power.clear();
    }

    pub fn is_powered(&self, entity: Entity) -> bool {
        self.main_power && *self.device_power.get(&entity).unwrap_or(&false)
    }

    pub fn set_device_power(&mut self, entity: Entity, powered: bool) {
        self.device_power.insert(entity, powered);
    }

    pub fn toggle_main_power(&mut self) {
        self.main_power = !self.main_power;
    }
}

#[derive(Event, Clone, Debug)]
pub enum PowerEvent {
    MainPowerOn,
    MainPowerOff,
    DevicePowered(Entity),
    DeviceUnpowered(Entity),
}

/// Marker: this entity draws power and participates in the power system.
#[derive(Component, Default)]
pub struct PoweredDevice {
    pub wattage: f32,
}

pub fn power_tick(
    mut power: ResMut<PowerSystem>,
    mut events: EventWriter<PowerEvent>,
    query: Query<(Entity, &PoweredDevice)>,
) {
    // Recompute consumption from currently registered powered devices.
    let mut total = 0.0;
    for (entity, device) in query.iter() {
        if power.is_powered(entity) {
            total += device.wattage;
        }
    }
    power.consumed_watts = total;

    // Future: thermal, brown-out, etc. can be driven from here.
    let _ = events;
}
