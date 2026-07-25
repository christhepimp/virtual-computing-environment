//! World systems that live inside the physics engine.
//!
//! These systems manage the entire state of the virtual computer.
//! They are the single source of truth for all hardware state.

pub mod power;
pub mod clock;
pub mod devices;
pub mod buses;
pub mod signals;
pub mod memory;
pub mod interrupts;
pub mod connections;
pub mod storage;
pub mod firmware;
pub mod motherboard;
pub mod cpu;
pub mod controllers;

use bevy::prelude::*;

use power::*;
use clock::*;
use devices::*;
use buses::*;
use signals::*;
use memory::*;
use interrupts::*;
use connections::*;
use storage::*;
use firmware::*;
use motherboard::*;
use cpu::*;
use controllers::*;

pub struct VirtualComputerPlugin;

impl Plugin for VirtualComputerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PowerSystem>()
            .init_resource::<ClockSystem>()
            .init_resource::<DeviceRegistry>()
            .init_resource::<BusSystem>()
            .init_resource::<SignalSystem>()
            .init_resource::<MemoryMapSystem>()
            .init_resource::<InterruptSystem>()
            .init_resource::<ConnectionSystem>()
            .init_resource::<StorageSystem>()
            .init_resource::<Firmware>()
            .add_event::<DeviceRegistered>()
            .add_event::<DeviceUnregistered>()
            .add_event::<PowerEvent>()
            .add_event::<SignalEvent>()
            .add_event::<InterruptEvent>()
            .add_event::<ConnectionEvent>()
            .add_event::<BusTransactionCompleted>()
            .add_event::<StorageIoEvent>()
            .add_systems(Startup, initialize_world_systems)
            .add_systems(
                Update,
                (
                    // Discovery & infrastructure
                    register_new_devices,
                    unregister_removed_devices,
                    power_tick,
                    clock_tick,
                    process_signals,
                    process_interrupts,
                    update_connections,
                    // Machine behavior
                    firmware_tick,
                    motherboard_bus_router,
                    cpu_tick,
                    keyboard_controller_tick,
                    mouse_controller_tick,
                    storage_controller_tick,
                    gpu_controller_tick,
                ),
            );
    }
}

fn initialize_world_systems(
    mut power: ResMut<PowerSystem>,
    mut clock: ResMut<ClockSystem>,
    mut registry: ResMut<DeviceRegistry>,
) {
    power.initialize();
    clock.initialize();
    registry.initialize();
    println!("World systems online. Physics world is the active runtime of the virtual computer.");
}
