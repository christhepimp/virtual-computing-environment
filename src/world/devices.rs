//! Device discovery and registration.
//!
//! Every hardware component that exists in the physics world automatically
//! registers here. The registry is the single source of truth for which
//! devices exist and their world-level identity.

use bevy::prelude::*;
use std::collections::HashMap;

/// Unique world-level identity for a registered device.
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub entity: Entity,
    pub name: String,
    pub kind: DeviceKind,
    pub registered: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    Case,
    Motherboard,
    Cpu,
    Ram,
    Gpu,
    Storage,
    Monitor,
    Keyboard,
    Mouse,
    PowerButton,
    PowerSupply,
    Bus,
    Unknown,
}

#[derive(Resource, Default)]
pub struct DeviceRegistry {
    pub devices: HashMap<Entity, DeviceInfo>,
    pub by_kind: HashMap<DeviceKind, Vec<Entity>>,
}

impl DeviceRegistry {
    pub fn initialize(&mut self) {
        self.devices.clear();
        self.by_kind.clear();
    }

    pub fn register(&mut self, entity: Entity, name: String, kind: DeviceKind) {
        let info = DeviceInfo {
            entity,
            name,
            kind,
            registered: true,
        };
        self.devices.insert(entity, info);
        self.by_kind.entry(kind).or_default().push(entity);
    }

    pub fn unregister(&mut self, entity: Entity) {
        if let Some(info) = self.devices.remove(&entity) {
            if let Some(list) = self.by_kind.get_mut(&info.kind) {
                list.retain(|&e| e != entity);
            }
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&DeviceInfo> {
        self.devices.get(&entity)
    }

    pub fn devices_of_kind(&self, kind: DeviceKind) -> &[Entity] {
        self.by_kind.get(&kind).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

#[derive(Event, Clone, Debug)]
pub struct DeviceRegistered {
    pub entity: Entity,
    pub kind: DeviceKind,
    pub name: String,
}

#[derive(Event, Clone, Debug)]
pub struct DeviceUnregistered {
    pub entity: Entity,
}

/// Component placed on a hardware entity so the world can discover and register it.
/// Adding this component is how new hardware joins the virtual computer.
#[derive(Component)]
pub struct RegisterDevice {
    pub name: String,
    pub kind: DeviceKind,
}

/// Internal marker once registration has occurred.
#[derive(Component)]
pub struct Registered;

/// Discover entities that request registration and enroll them in the world systems.
pub fn register_new_devices(
    mut commands: Commands,
    mut registry: ResMut<DeviceRegistry>,
    mut power: ResMut<super::power::PowerSystem>,
    mut clock: ResMut<super::clock::ClockSystem>,
    mut events: EventWriter<DeviceRegistered>,
    query: Query<(Entity, &RegisterDevice), Without<Registered>>,
) {
    for (entity, req) in query.iter() {
        registry.register(entity, req.name.clone(), req.kind);
        power.set_device_power(entity, false);
        // Default clock domain; specific devices can override later.
        clock.register_device(entity, 0);

        commands.entity(entity).insert(Registered);

        events.send(DeviceRegistered {
            entity,
            kind: req.kind,
            name: req.name.clone(),
        });

        println!(
            "Device registered: {} ({:?}) [entity {:?}]",
            req.name, req.kind, entity
        );
    }
}

/// Clean up when a registered device is removed from the world.
pub fn unregister_removed_devices(
    mut registry: ResMut<DeviceRegistry>,
    mut power: ResMut<super::power::PowerSystem>,
    mut clock: ResMut<super::clock::ClockSystem>,
    mut events: EventWriter<DeviceUnregistered>,
    mut removed: RemovedComponents<Registered>,
) {
    for entity in removed.read() {
        registry.unregister(entity);
        power.device_power.remove(&entity);
        clock.unregister_device(entity);
        events.send(DeviceUnregistered { entity });
        println!("Device unregistered: {:?}", entity);
    }
}
