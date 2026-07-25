//! Storage adapter — block I/O via StorageSystem.

use bevy::prelude::Entity;

use crate::world::storage::StorageSystem;

#[derive(Clone, Debug)]
pub struct BlockDeviceView {
    pub entity_bits: u64,
    pub sectors: u64,
}

#[derive(Default, Debug)]
pub struct StorageAdapter {
    pub devices: Vec<BlockDeviceView>,
}

impl StorageAdapter {
    pub fn sync_from_world(&mut self, storage: &StorageSystem) {
        self.devices = storage
            .devices
            .iter()
            .map(|(e, d)| BlockDeviceView {
                entity_bits: e.to_bits(),
                sectors: d.sectors,
            })
            .collect();
    }

    pub fn read(
        storage: &StorageSystem,
        entity: Entity,
        lba: u64,
        count: u64,
    ) -> Option<Vec<u8>> {
        storage.read_sectors(entity, lba, count)
    }

    pub fn write(storage: &mut StorageSystem, entity: Entity, lba: u64, data: &[u8]) -> bool {
        storage.write_sectors(entity, lba, data)
    }
}
