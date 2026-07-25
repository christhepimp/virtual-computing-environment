//! Interrupt adapter — exposes pending IRQs from InterruptSystem to the guest.

use crate::world::interrupts::InterruptSystem;

#[derive(Default, Debug)]
pub struct InterruptAdapter {
    pub last_seen: Vec<u8>,
}

impl InterruptAdapter {
    /// Snapshot pending vectors for the guest without clearing world state
    /// permanently; transport/ack path acknowledges through the world.
    pub fn pull_pending_from_world(&mut self, interrupts: &InterruptSystem) -> Vec<u8> {
        let vectors: Vec<u8> = interrupts.pending.iter().map(|p| p.vector).collect();
        self.last_seen = vectors.clone();
        vectors
    }
}
