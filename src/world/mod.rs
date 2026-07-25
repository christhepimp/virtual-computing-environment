//! World systems that live inside the physics engine.
//!
//! These systems manage the entire state of the virtual computer.
//! They are the single source of truth for all hardware state.
//! Hardware entities register themselves; they do not maintain
//! isolated state outside these systems.
//!
//! The future Linux emulator will communicate only through the
//! interfaces and services these world systems provide.

pub mod power;
pub mod clock;
pub mod devices;
pub mod buses;
pub mod signals;
pub mod memory;
pub mod interrupts;
pub mod connections;

use bevy::prelude::*;

use power::*;
use clock::*;
use devices::*;
use buses::*;
use signals::*;
use memory::*;
use interrupts::*;
use connections::*;

/// Plugin that installs all virtual-computer world systems.
/// Everything runs inside the physics world.
pub struct VirtualComputerPlugin;

impl Plugin for VirtualComputerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Core world resources (single source of truth)
            .init_resource::<PowerSystem>()
            .init_resource::<ClockSystem>()
            .init_resource::<DeviceRegistry>()
            .init_resource::<BusSystem>()
            .init_resource::<SignalSystem>()
            .init_resource::<MemoryMapSystem>()
            .init_resource::<InterruptSystem>()
            .init_resource::<ConnectionSystem>()
            // Events
            .add_event::<DeviceRegistered>()
            .add_event::<DeviceUnregistered>()
            .add_event::<PowerEvent>()
            .add_event::<SignalEvent>()
            .add_event::<InterruptEvent>()
            .add_event::<ConnectionEvent>()
            // Systems
            .add_systems(Startup, initialize_world_systems)
            .add_systems(
                Update,
                (
                    register_new_devices,
                    unregister_removed_devices,
                    power_tick,
                    clock_tick,
                    process_signals,
                    process_interrupts,
                    update_connections,
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
