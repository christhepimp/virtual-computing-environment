//! Memory adapter — guest memory access goes through MemoryMapSystem.

use crate::world::memory::MemoryMapSystem;

#[derive(Clone, Debug, Default)]
pub struct MemoryRegionView {
    pub base: u64,
    pub size: u64,
    pub name: String,
    pub is_ram: bool,
}

#[derive(Default, Debug)]
pub struct MemoryAdapter {
    pub regions: Vec<MemoryRegionView>,
}

impl MemoryAdapter {
    pub fn sync_map_from_world(&mut self, memory: &MemoryMapSystem) {
        self.regions = memory
            .regions
            .values()
            .map(|r| MemoryRegionView {
                base: r.base,
                size: r.size,
                name: r.name.clone(),
                is_ram: r.is_ram,
            })
            .collect();
    }

    pub fn read_through_world(
        &self,
        memory: &MemoryMapSystem,
        addr: u64,
        len: usize,
    ) -> Option<Vec<u8>> {
        memory.read_ram(addr, len)
    }

    pub fn write_through_world(
        &self,
        memory: &mut MemoryMapSystem,
        addr: u64,
        data: &[u8],
    ) -> bool {
        memory.write_ram(addr, data)
    }
}
