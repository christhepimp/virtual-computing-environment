//! Emulator integration — non-owning guest software path.
//!
//! QEMU is introduced through an Integration Layer that translates the
//! virtual computer's world interfaces into the services an emulator needs.
//! The physics world and world systems remain the sole authority over
//! hardware state. The emulator never owns or independently simulates
//! motherboard, CPU silicon, RAM, storage, or devices.
//!
//! Layout:
//!   layer     — orchestration and lifecycle of the guest process
//!   adapters  — one adapter per hardware interface class
//!   qemu      — process launch / transport for QEMU
//!   config    — minimal machine configuration for boot experiments

pub mod layer;
pub mod adapters;
pub mod qemu;
pub mod config;

use bevy::prelude::*;

use layer::*;

/// Marker: a software process running on the virtual computer.
#[derive(Component)]
pub struct EmulatorProcess;

pub struct EmulatorPlugin;

impl Plugin for EmulatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmulatorIntegration>()
            .init_resource::<config::MinimalMachineConfig>()
            .add_event::<EmulatorEvent>()
            .add_systems(
                Update,
                (
                    integration_lifecycle,
                    sync_power_adapter,
                    sync_clock_adapter,
                    sync_memory_adapter,
                    sync_interrupt_adapter,
                    sync_storage_adapter,
                    sync_input_adapter,
                    pump_qemu_transport,
                ),
            );
    }
}

pub use layer::{EmulatorEvent, EmulatorIntegration, EmulatorState};
