//! Power system — world-level authority for power state.

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct PowerSystem {
    pub main_power: bool,
    pub device_power: HashMap<Entity, bool>,
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

    pub fn set_main_power(&mut self, on: bool, events: &mut EventWriter<PowerEvent>) {
        if self.main_power == on {
            return;
        }
        self.main_power = on;
        if on {
            events.send(PowerEvent::MainPowerOn);
        } else {
            events.send(PowerEvent::MainPowerOff);
        }
    }

    pub fn toggle_main_power(&mut self, events: &mut EventWriter<PowerEvent>) {
        self.set_main_power(!self.main_power, events);
    }
}

#[derive(Event, Clone, Debug)]
pub enum PowerEvent {
    MainPowerOn,
    MainPowerOff,
    DevicePowered(Entity),
    DeviceUnpowered(Entity),
}

#[derive(Component, Default)]
pub struct PoweredDevice {
    pub wattage: f32,
}

pub fn power_tick(
    mut power: ResMut<PowerSystem>,
    query: Query<(Entity, &PoweredDevice)>,
) {
    let mut total = 0.0;
    for (entity, device) in query.iter() {
        if power.is_powered(entity) {
            total += device.wattage;
        }
    }
    power.consumed_watts = total;
}
