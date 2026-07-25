//! Per-device state machines advanced only from world inputs
//! (power, PowerGood, ClockEnable, firmware phase).

use bevy::prelude::*;

use super::clock::ClockSystem;
use super::devices::{DeviceKind, DeviceRegistry, Registered};
use super::firmware::{Firmware, FirmwarePhase};
use super::lifecycle::{DeviceLifecycle, DevicePhase};
use super::power::PowerSystem;
use super::signals::{SignalId, SignalSystem};
use crate::hardware::{
    Cpu, Gpu, Keyboard, Monitor, Motherboard, Mouse, Ram, Storage,
};

fn advance_common(
    life: &mut DeviceLifecycle,
    powered: bool,
    power_good: bool,
    clock_enable: bool,
    name: &str,
) {
    if !powered || !power_good {
        if life.phase != DevicePhase::Offline && life.phase != DevicePhase::Failed {
            life.force_offline();
            println!("[{name}] Offline (power lost)");
        }
        return;
    }

    match life.phase {
        DevicePhase::Offline => {
            life.phase = DevicePhase::PowerApplied;
            life.init_ticks = 0;
            println!("[{name}] Power applied");
        }
        DevicePhase::PowerApplied => {
            life.phase = DevicePhase::ResetHold;
            println!("[{name}] Reset hold");
        }
        DevicePhase::ResetHold => {
            if clock_enable {
                life.phase = DevicePhase::Initializing;
                life.init_ticks = 0;
                println!("[{name}] Initializing");
            }
        }
        DevicePhase::Initializing => {
            life.init_ticks += 1;
            if life.init_ticks >= life.required_init_ticks {
                life.phase = DevicePhase::Online;
                println!("[{name}] Online");
            }
        }
        DevicePhase::Online => {
            life.phase = DevicePhase::Ready;
            println!("[{name}] Ready");
        }
        DevicePhase::Ready | DevicePhase::Error | DevicePhase::Failed => {}
    }
}

pub fn motherboard_fsm(
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    mut query: Query<(Entity, &mut DeviceLifecycle), (With<Motherboard>, With<Registered>)>,
) {
    let pg = signals.is_asserted(SignalId::PowerGood);
    let ce = signals.is_asserted(SignalId::ClockEnable);
    for (entity, mut life) in query.iter_mut() {
        advance_common(&mut life, power.is_powered(entity), pg, ce, "Motherboard");
    }
}

pub fn cpu_fsm(
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    mut clock: ResMut<ClockSystem>,
    mut query: Query<(Entity, &mut DeviceLifecycle), (With<Cpu>, With<Registered>)>,
) {
    let pg = signals.is_asserted(SignalId::PowerGood);
    let ce = signals.is_asserted(SignalId::ClockEnable);
    for (entity, mut life) in query.iter_mut() {
        let was = life.phase;
        advance_common(&mut life, power.is_powered(entity), pg, ce, "CPU");
        // CPU only receives clock when Online/Ready.
        if let Some(dc) = clock.device_clocks.get_mut(&entity) {
            dc.enabled = matches!(life.phase, DevicePhase::Online | DevicePhase::Ready) && ce;
        }
        if was != life.phase && life.phase == DevicePhase::Ready {
            println!("[CPU] Clock domain active");
        }
    }
}

pub fn ram_fsm(
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    mut query: Query<(Entity, &mut DeviceLifecycle), (With<Ram>, With<Registered>)>,
) {
    let pg = signals.is_asserted(SignalId::PowerGood);
    let ce = signals.is_asserted(SignalId::ClockEnable);
    for (entity, mut life) in query.iter_mut() {
        advance_common(&mut life, power.is_powered(entity), pg, ce, "RAM");
    }
}

pub fn storage_fsm(
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    mut query: Query<(Entity, &mut DeviceLifecycle), (With<Storage>, With<Registered>)>,
) {
    let pg = signals.is_asserted(SignalId::PowerGood);
    let ce = signals.is_asserted(SignalId::ClockEnable);
    for (entity, mut life) in query.iter_mut() {
        advance_common(&mut life, power.is_powered(entity), pg, ce, "Storage");
    }
}

pub fn gpu_fsm(
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    mut query: Query<(Entity, &mut DeviceLifecycle), (With<Gpu>, With<Registered>)>,
) {
    let pg = signals.is_asserted(SignalId::PowerGood);
    let ce = signals.is_asserted(SignalId::ClockEnable);
    for (entity, mut life) in query.iter_mut() {
        advance_common(&mut life, power.is_powered(entity), pg, ce, "GPU");
    }
}

pub fn keyboard_fsm(
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    mut query: Query<(Entity, &mut DeviceLifecycle), (With<Keyboard>, With<Registered>)>,
) {
    let pg = signals.is_asserted(SignalId::PowerGood);
    let ce = signals.is_asserted(SignalId::ClockEnable);
    for (entity, mut life) in query.iter_mut() {
        advance_common(&mut life, power.is_powered(entity), pg, ce, "Keyboard");
    }
}

pub fn mouse_fsm(
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    mut query: Query<(Entity, &mut DeviceLifecycle), (With<Mouse>, With<Registered>)>,
) {
    let pg = signals.is_asserted(SignalId::PowerGood);
    let ce = signals.is_asserted(SignalId::ClockEnable);
    for (entity, mut life) in query.iter_mut() {
        advance_common(&mut life, power.is_powered(entity), pg, ce, "Mouse");
    }
}

pub fn monitor_fsm(
    power: Res<PowerSystem>,
    signals: Res<SignalSystem>,
    mut query: Query<(Entity, &mut DeviceLifecycle), (With<Monitor>, With<Registered>)>,
) {
    let pg = signals.is_asserted(SignalId::PowerGood);
    // Monitor does not require system clock enable to show signal, but needs power-good.
    for (entity, mut life) in query.iter_mut() {
        advance_common(&mut life, power.is_powered(entity), pg, true, "Monitor");
    }
}

/// Firmware only enables clocks after critical devices are Online/Ready.
pub fn firmware_gate_clocks(
    mut firmware: ResMut<Firmware>,
    mut signals: ResMut<SignalSystem>,
    mut power: ResMut<PowerSystem>,
    registry: Res<DeviceRegistry>,
    life_query: Query<(Entity, &DeviceLifecycle), With<Registered>>,
) {
    if firmware.phase == FirmwarePhase::Off || firmware.phase == FirmwarePhase::Halted {
        return;
    }

    // Ensure registered non-PSU devices get device_power once rails are good.
    if signals.is_asserted(SignalId::PowerGood) {
        for (entity, info) in registry.devices.iter() {
            if matches!(
                info.kind,
                DeviceKind::PowerSupply | DeviceKind::PowerButton | DeviceKind::Case
            ) {
                continue;
            }
            power.set_device_power(*entity, true);
        }
    }

    // After discovery, wait until CPU + RAM + Motherboard are at least Online
    // before asserting ClockEnable (if not already).
    if matches!(
        firmware.phase,
        FirmwarePhase::MemoryInit | FirmwarePhase::StorageInit | FirmwarePhase::DeviceDiscovery
    ) {
        let mut mb_ok = false;
        let mut cpu_ok = false;
        let mut ram_ok = false;
        for (entity, life) in life_query.iter() {
            if let Some(info) = registry.get(entity) {
                let readyish = matches!(life.phase, DevicePhase::Online | DevicePhase::Ready);
                match info.kind {
                    DeviceKind::Motherboard => mb_ok |= readyish || life.phase == DevicePhase::Initializing || life.phase == DevicePhase::ResetHold || life.phase == DevicePhase::PowerApplied,
                    DeviceKind::Cpu => cpu_ok |= readyish || matches!(life.phase, DevicePhase::Initializing | DevicePhase::ResetHold | DevicePhase::PowerApplied),
                    DeviceKind::Ram => ram_ok |= readyish || matches!(life.phase, DevicePhase::Initializing | DevicePhase::ResetHold | DevicePhase::PowerApplied),
                    _ => {}
                }
            }
        }
        // Clock enable once powered devices have left Offline.
        if signals.is_asserted(SignalId::PowerGood) && mb_ok && cpu_ok && ram_ok {
            if !signals.is_asserted(SignalId::ClockEnable) {
                signals.assert(SignalId::ClockEnable);
                println!("[Firmware] ClockEnable asserted");
            }
        }
    }

    let _ = &mut firmware;
}
