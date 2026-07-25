//! Power adapter — guest sees power rails through PowerSystem + signals.

use crate::world::power::PowerSystem;
use crate::world::signals::{SignalId, SignalSystem};

#[derive(Default, Debug)]
pub struct PowerAdapter {
    pub main_power: bool,
    pub power_good: bool,
    pub clock_enable: bool,
}

impl PowerAdapter {
    pub fn sync_from_world(&mut self, power: &PowerSystem, signals: &SignalSystem) {
        self.main_power = power.main_power;
        self.power_good = signals.is_asserted(SignalId::PowerGood);
        self.clock_enable = signals.is_asserted(SignalId::ClockEnable);
    }
}
