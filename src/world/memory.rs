//! Memory mapping and RAM backing store.
//!
//! The world owns the address map and the actual memory contents for RAM.
//! Devices and the future OS access memory only through bus transactions
//! or the memory map interface — never by reaching into another entity.

use bevy::prelude::*;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug)]
pub struct MemoryRegion {
    pub base: u64,
    pub size: u64,
    pub owner: Entity,
    pub name: String,
    /// If true, this region is backed by the world RAM store.
    pub is_ram: bool,
}

#[derive(Resource, Default)]
pub struct MemoryMapSystem {
    pub regions: BTreeMap<u64, MemoryRegion>,
    /// Authoritative RAM contents keyed by owner entity.
    pub ram_stores: HashMap<Entity, Vec<u8>>,
}

impl MemoryMapSystem {
    pub fn map(
        &mut self,
        base: u64,
        size: u64,
        owner: Entity,
        name: impl Into<String>,
        is_ram: bool,
    ) {
        if is_ram {
            self.ram_stores
                .entry(owner)
                .or_insert_with(|| vec![0u8; size.min(64 * 1024 * 1024) as usize]); // cap for safety
        }
        self.regions.insert(
            base,
            MemoryRegion {
                base,
                size,
                owner,
                name: name.into(),
                is_ram,
            },
        );
    }

    pub fn unmap_owner(&mut self, owner: Entity) {
        self.regions.retain(|_, r| r.owner != owner);
        self.ram_stores.remove(&owner);
    }

    pub fn resolve(&self, address: u64) -> Option<&MemoryRegion> {
        self.regions
            .range(..=address)
            .next_back()
            .map(|(_, r)| r)
            .filter(|r| address < r.base + r.size)
    }

    /// Read from RAM-backed region. Returns None if not RAM or OOB.
    pub fn read_ram(&self, address: u64, len: usize) -> Option<Vec<u8>> {
        let region = self.resolve(address)?;
        if !region.is_ram {
            return None;
        }
        let store = self.ram_stores.get(&region.owner)?;
        let offset = (address - region.base) as usize;
        if offset + len > store.len() {
            return None;
        }
        Some(store[offset..offset + len].to_vec())
    }

    /// Write to RAM-backed region.
    pub fn write_ram(&mut self, address: u64, data: &[u8]) -> bool {
        let region = match self.resolve(address) {
            Some(r) if r.is_ram => r.clone(),
            _ => return false,
        };
        let store = match self.ram_stores.get_mut(&region.owner) {
            Some(s) => s,
            None => return false,
        };
        let offset = (address - region.base) as usize;
        if offset + data.len() > store.len() {
            return false;
        }
        store[offset..offset + data.len()].copy_from_slice(data);
        true
    }
}

/// Component: this device exposes a memory-mapped region.
#[derive(Component)]
pub struct MemoryMappedRegion {
    pub base: u64,
    pub size: u64,
    pub is_ram: bool,
}
