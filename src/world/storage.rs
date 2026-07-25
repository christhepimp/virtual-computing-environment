//! Block storage services.
//!
//! Storage devices provide sector/block read and write through the world.
//! Access is mediated by bus transactions or explicit block requests so that
//! an OS can treat this like a real disk controller.

use bevy::prelude::*;
use std::collections::HashMap;

pub const SECTOR_SIZE: usize = 512;

#[derive(Clone, Debug)]
pub struct BlockDevice {
    pub entity: Entity,
    pub sectors: u64,
    pub data: Vec<u8>,
}

#[derive(Resource, Default)]
pub struct StorageSystem {
    pub devices: HashMap<Entity, BlockDevice>,
}

impl StorageSystem {
    pub fn register(&mut self, entity: Entity, sector_count: u64) {
        let size = (sector_count as usize).saturating_mul(SECTOR_SIZE);
        // Cap for foundation builds
        let size = size.min(16 * 1024 * 1024);
        self.devices.insert(
            entity,
            BlockDevice {
                entity,
                sectors: (size / SECTOR_SIZE) as u64,
                data: vec![0u8; size],
            },
        );
    }

    pub fn unregister(&mut self, entity: Entity) {
        self.devices.remove(&entity);
    }

    pub fn read_sectors(
        &self,
        entity: Entity,
        lba: u64,
        count: u64,
    ) -> Option<Vec<u8>> {
        let dev = self.devices.get(&entity)?;
        if lba + count > dev.sectors {
            return None;
        }
        let start = (lba as usize) * SECTOR_SIZE;
        let end = start + (count as usize) * SECTOR_SIZE;
        Some(dev.data[start..end].to_vec())
    }

    pub fn write_sectors(&mut self, entity: Entity, lba: u64, data: &[u8]) -> bool {
        let dev = match self.devices.get_mut(&entity) {
            Some(d) => d,
            None => return false,
        };
        if data.len() % SECTOR_SIZE != 0 {
            return false;
        }
        let count = (data.len() / SECTOR_SIZE) as u64;
        if lba + count > dev.sectors {
            return false;
        }
        let start = (lba as usize) * SECTOR_SIZE;
        dev.data[start..start + data.len()].copy_from_slice(data);
        true
    }
}

/// Component marking a block storage device.
#[derive(Component)]
pub struct BlockStorage {
    pub sector_count: u64,
}

#[derive(Event, Clone, Debug)]
pub struct StorageIoEvent {
    pub entity: Entity,
    pub lba: u64,
    pub write: bool,
    pub success: bool,
}
