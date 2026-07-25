//! World systems — single source of truth for machine state.

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
pub mod lifecycle;
pub mod power_sequence;
pub mod device_fsm;

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
use power_sequence::*;
use device_fsm::*;

pub struct VirtualComputerPlugin;

impl Plugin for VirtualComputerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PowerSystem>()
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
                    // Power path
                    power_button_system,
                    power_supply_system,
                    enforce_rail_power,
                    power_tick,
                    // Registration & clocks
                    register_new_devices,
                    unregister_removed_devices,
                    clock_tick,
                    process_signals,
                    process_interrupts,
                    update_connections,
                    // Device state machines
                    motherboard_fsm,
                    cpu_fsm,
                    ram_fsm,
                    storage_fsm,
                    gpu_fsm,
                    keyboard_fsm,
                    mouse_fsm,
                    monitor_fsm,
                    firmware_gate_clocks,
                    // Firmware & machine services
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
    println!("World systems online.");
}
