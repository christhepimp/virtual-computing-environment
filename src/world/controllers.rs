//! Device controllers — keyboard, mouse, GPU, storage controllers.
//!
//! Controllers sit between the buses/interrupts and the device-specific
//! logic. They expose the MMIO/IRQ surface an OS would program.

use bevy::prelude::*;

use super::interrupts::InterruptSystem;
use super::power::PowerSystem;
use super::storage::{StorageIoEvent, StorageSystem};
use crate::hardware::{Gpu, Keyboard, Mouse, Storage};
use super::devices::Registered;

/// Keyboard controller: raises interrupt on key events (placeholder queue).
#[derive(Component, Default)]
pub struct KeyboardController {
    pub scancode_queue: Vec<u8>,
}

/// Mouse controller: simple packet queue.
#[derive(Component, Default)]
pub struct MouseController {
    pub packet_queue: Vec<[u8; 3]>,
}

/// GPU controller: framebuffer base (MMIO) placeholder.
#[derive(Component, Default)]
pub struct GpuController {
    pub framebuffer_addr: u64,
    pub width: u32,
    pub height: u32,
}

/// Storage controller: ATA-like command surface (very simplified).
#[derive(Component, Default)]
pub struct StorageController {
    pub last_lba: u64,
    pub last_count: u64,
}

pub fn keyboard_controller_tick(
    power: Res<PowerSystem>,
    mut interrupts: ResMut<InterruptSystem>,
    mut query: Query<(Entity, &mut KeyboardController), (With<Keyboard>, With<Registered>)>,
) {
    for (entity, mut ctl) in query.iter_mut() {
        if !power.is_powered(entity) {
            continue;
        }
        if let Some(_scancode) = ctl.scancode_queue.pop() {
            // IRQ 1 for keyboard
            interrupts.raise(1, entity);
        }
    }
}

pub fn mouse_controller_tick(
    power: Res<PowerSystem>,
    mut interrupts: ResMut<InterruptSystem>,
    mut query: Query<(Entity, &mut MouseController), (With<Mouse>, With<Registered>)>,
) {
    for (entity, mut ctl) in query.iter_mut() {
        if !power.is_powered(entity) {
            continue;
        }
        if ctl.packet_queue.pop().is_some() {
            interrupts.raise(12, entity);
        }
    }
}

pub fn storage_controller_tick(
    power: Res<PowerSystem>,
    storage: Res<StorageSystem>,
    mut events: EventWriter<StorageIoEvent>,
    query: Query<(Entity, &StorageController), (With<Storage>, With<Registered>)>,
) {
    // Controllers become active when powered; actual commands will come
    // from MMIO writes issued by guest software via the bus.
    for (entity, _ctl) in query.iter() {
        if !power.is_powered(entity) {
            continue;
        }
        let _ = (storage, &mut events, entity);
    }
}

pub fn gpu_controller_tick(
    power: Res<PowerSystem>,
    query: Query<(Entity, &GpuController), (With<Gpu>, With<Registered>)>,
) {
    for (entity, _ctl) in query.iter() {
        if !power.is_powered(entity) {
            continue;
        }
        // Future: vsync IRQ, command ring, etc.
    }
}
