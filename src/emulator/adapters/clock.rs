//! Clock adapter — guest timers observe world ClockSystem.

use crate::world::clock::ClockSystem;

#[derive(Default, Debug)]
pub struct ClockAdapter {
    pub master_ticks: u64,
    pub master_hz: u64,
}

impl ClockAdapter {
    pub fn sync_from_world(&mut self, clock: &ClockSystem) {
        self.master_ticks = clock.master_ticks;
        self.master_hz = clock.master_hz;
    }
}
