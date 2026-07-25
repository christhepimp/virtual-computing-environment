//! Memory mapping system.
//!
//! The world owns the authoritative map of address ranges to devices.
//! Software (future emulator) and other devices resolve addresses here.

use bevy::prelude::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct MemoryRegion {
    pub base: u64,
    pub size: u64,
    pub owner: Entity,
    pub name: String,
}

#[derive(Resource, Default)]
pub struct MemoryMapSystem {
    /// Sorted by base address for lookup.
    pub regions: BTreeMap<u64, MemoryRegion>,
}

impl MemoryMapSystem {
    pub fn map(&mut self, base: u64, size: u64, owner: Entity, name: impl Into<String>) {
        self.regions.insert(
            base,
            MemoryRegion {
                base,
                size,
                owner,
                name: name.into(),
            },
        );
    }

    pub fn unmap_owner(&mut self, owner: Entity) {
        self.regions.retain(|_, r| r.owner != owner);
    }

    pub fn resolve(&self, address: u64) -> Option<&MemoryRegion> {
        self.regions
            .range(..=address)
            .next_back()
            .map(|(_, r)| r)
            .filter(|r| address < r.base + r.size)
    }
}

/// Component: this device exposes a memory-mapped region.
#[derive(Component)]
pub struct MemoryMappedRegion {
    pub base: u64,
    pub size: u64,
}
