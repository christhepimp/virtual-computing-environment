//! Device discovery and registration.
//!
//! Every hardware component that exists in the physics world automatically
//! registers here. On registration, interface components are wired into
//! the corresponding world systems (buses, memory map, storage, clocks).

use bevy::prelude::*;
use std::collections::HashMap;

use super::buses::{BusAttachment, BusSystem};
use super::clock::{ClockedDevice, ClockSystem};
use super::memory::{MemoryMapSystem, MemoryMappedRegion};
use super::power::PowerSystem;
use super::storage::{BlockStorage, StorageSystem};

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

#[derive(Component)]
pub struct RegisterDevice {
    pub name: String,
    pub kind: DeviceKind,
}

#[derive(Component)]
pub struct Registered;

pub fn register_new_devices(
    mut commands: Commands,
    mut registry: ResMut<DeviceRegistry>,
    mut power: ResMut<PowerSystem>,
    mut clock: ResMut<ClockSystem>,
    mut buses: ResMut<BusSystem>,
    mut memory: ResMut<MemoryMapSystem>,
    mut storage: ResMut<StorageSystem>,
    mut events: EventWriter<DeviceRegistered>,
    query: Query<
        (
            Entity,
            &RegisterDevice,
            Option<&BusAttachment>,
            Option<&MemoryMappedRegion>,
            Option<&ClockedDevice>,
            Option<&BlockStorage>,
        ),
        Without<Registered>,
    >,
) {
    for (entity, req, bus_att, mmio, clocked, block) in query.iter() {
        registry.register(entity, req.name.clone(), req.kind);
        power.set_device_power(entity, false);

        let hz = clocked.map(|c| c.hz).unwrap_or(0);
        clock.register_device(entity, hz);

        if let Some(att) = bus_att {
            for &bus_id in &att.buses {
                buses.attach(bus_id, entity);
            }
        }

        if let Some(region) = mmio {
            memory.map(
                region.base,
                region.size,
                entity,
                req.name.clone(),
                region.is_ram,
            );
        }

        if let Some(bs) = block {
            storage.register(entity, bs.sector_count);
        }

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

pub fn unregister_removed_devices(
    mut registry: ResMut<DeviceRegistry>,
    mut power: ResMut<PowerSystem>,
    mut clock: ResMut<ClockSystem>,
    mut memory: ResMut<MemoryMapSystem>,
    mut storage: ResMut<StorageSystem>,
    mut events: EventWriter<DeviceUnregistered>,
    mut removed: RemovedComponents<Registered>,
) {
    for entity in removed.read() {
        registry.unregister(entity);
        power.device_power.remove(&entity);
        clock.unregister_device(entity);
        memory.unmap_owner(entity);
        storage.unregister(entity);
        events.send(DeviceUnregistered { entity });
        println!("Device unregistered: {:?}", entity);
    }
}
