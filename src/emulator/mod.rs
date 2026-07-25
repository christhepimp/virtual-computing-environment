//! Emulator integration — non-owning guest software path.
//!
//! QEMU is guest execution only. World systems remain the authority.

pub mod layer;
pub mod adapters;
pub mod qemu;
pub mod config;
pub mod boot;

use bevy::prelude::*;

use boot::*;
use layer::*;

#[derive(Component)]
pub struct EmulatorProcess;

pub struct EmulatorPlugin;

impl Plugin for EmulatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmulatorIntegration>()
            .init_resource::<config::MinimalMachineConfig>()
            .init_resource::<BootExperiment>()
            .add_event::<EmulatorEvent>()
            .add_systems(
                Update,
                (
                    integration_lifecycle,
                    boot_experiment_tick,
                    sync_power_adapter,
                    sync_clock_adapter,
                    sync_memory_adapter,
                    sync_interrupt_adapter,
                    sync_storage_adapter,
                    sync_input_adapter,
                    pump_qemu_transport,
                    log_adapter_traffic,
                ),
            );
    }
}

pub use boot::{BootExperiment, BootPhase};
pub use layer::{EmulatorEvent, EmulatorIntegration, EmulatorState};

fn log_adapter_traffic(
    integ: Res<EmulatorIntegration>,
    mut events: EventReader<EmulatorEvent>,
) {
    for ev in events.read() {
        match ev {
            EmulatorEvent::GuestMemoryAccess { addr, write, len } => {
                println!(
                    "[Path][Memory] {} addr={:#x} len={}",
                    if *write { "WRITE" } else { "READ" },
                    addr,
                    len
                );
            }
            EmulatorEvent::GuestInterrupt { vector } => {
                println!("[Path][IRQ] vector={vector}");
            }
            EmulatorEvent::Started => {
                println!(
                    "[Path] Emulator started (transport={})",
                    integ.transport.mode_name()
                );
            }
            EmulatorEvent::Stopped => println!("[Path] Emulator stopped"),
            EmulatorEvent::Paused => println!("[Path] Emulator paused"),
            EmulatorEvent::Resumed => println!("[Path] Emulator resumed"),
            EmulatorEvent::Crashed(msg) => println!("[Path] Emulator crashed: {msg}"),
        }
    }

    // Clock path visibility
    if integ.clock.master_ticks > 0 && integ.clock.master_ticks % 256 == 0 {
        println!(
            "[Path][Clock] master_ticks={} hz={}",
            integ.clock.master_ticks, integ.clock.master_hz
        );
    }

    // Power path visibility when running
    if matches!(integ.state, EmulatorState::Running | EmulatorState::Paused)
        && integ.power.main_power
    {
        // low-noise: only when ticks align with clock log
        if integ.clock.master_ticks % 256 == 0 {
            println!(
                "[Path][Power] main={} power_good={} clock_enable={}",
                integ.power.main_power, integ.power.power_good, integ.power.clock_enable
            );
        }
    }
}
