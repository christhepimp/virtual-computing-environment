//! Power-on / power-off sequence.
//!
//! Order of operations (power-on):
//!   1. Power button edge
//!   2. Power supply spin-up
//!   3. Rails stable → main_power true + PowerGood signal path begins
//!   4. Devices see power and enter their state machines
//!   5. Firmware runs POST → discovery → init → Ready
//!
//! Power-off or PSU failure forces every device offline through PowerSystem.

use bevy::prelude::*;

use super::devices::{DeviceKind, DeviceRegistry, Registered};
use super::lifecycle::{DeviceLifecycle, DevicePhase, PowerButtonState, PowerSupplyState};
use super::power::{PowerEvent, PowerSystem};
use super::signals::{SignalId, SignalSystem};
use crate::hardware::{PowerButton, PowerSupply};

/// Detect power-button press edges and request main power toggle.
pub fn power_button_system(
    mut power: ResMut<PowerSystem>,
    mut events: EventWriter<PowerEvent>,
    mut query: Query<&mut PowerButtonState, (With<PowerButton>, With<Registered>)>,
) {
    for mut btn in query.iter_mut() {
        if btn.pressed && !btn.last_pressed {
            power.toggle_main_power(&mut events);
            println!(
                "[PowerButton] Pressed — main power now {}",
                if power.main_power { "ON" } else { "OFF" }
            );
        }
        btn.last_pressed = btn.pressed;
        // Consume the press so it is edge-triggered only.
        btn.pressed = false;
    }
}

/// Power supply reacts to main power requests: spin up or shut down rails.
pub fn power_supply_system(
    power: Res<PowerSystem>,
    mut signals: ResMut<SignalSystem>,
    mut query: Query<(&mut PowerSupplyState, &mut DeviceLifecycle), (With<PowerSupply>, With<Registered>)>,
) {
    for (mut psu, mut life) in query.iter_mut() {
        if power.main_power {
            match life.phase {
                DevicePhase::Offline | DevicePhase::Failed | DevicePhase::Error => {
                    psu.spinning_up = true;
                    psu.rails_stable = false;
                    psu.spin_ticks = 0;
                    life.phase = DevicePhase::PowerApplied;
                    println!("[PSU] Spinning up");
                }
                DevicePhase::PowerApplied | DevicePhase::Initializing if psu.spinning_up => {
                    psu.spin_ticks += 1;
                    life.phase = DevicePhase::Initializing;
                    if psu.spin_ticks >= psu.required_spin_ticks {
                        psu.spinning_up = false;
                        psu.rails_stable = true;
                        life.phase = DevicePhase::Ready;
                        signals.assert(SignalId::PowerGood);
                        println!("[PSU] Rails stable — PowerGood asserted");
                    }
                }
                _ => {}
            }
        } else {
            // Power removed — rails collapse immediately.
            if psu.rails_stable || psu.spinning_up || life.phase != DevicePhase::Offline {
                psu.spinning_up = false;
                psu.rails_stable = false;
                psu.spin_ticks = 0;
                life.force_offline();
                signals.deassert(SignalId::PowerGood);
                signals.deassert(SignalId::ClockEnable);
                println!("[PSU] Rails down — PowerGood deasserted");
            }
        }
    }
}

/// When rails are not stable, no device may remain powered.
pub fn enforce_rail_power(
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    registry: Res<DeviceRegistry>,
    mut power_mut: ResMut<PowerSystem>,
    psu_query: Query<&PowerSupplyState, With<PowerSupply>>,
) {
    let rails_ok = psu_query.iter().any(|p| p.rails_stable) && power.main_power;

    if !rails_ok {
        // Strip device power; leave main_power flag as the button/PSU intent.
        for (entity, info) in registry.devices.iter() {
            if info.kind == DeviceKind::PowerSupply || info.kind == DeviceKind::PowerButton {
                continue;
            }
            power_mut.set_device_power(*entity, false);
        }
        let _ = signals;
    }
}
