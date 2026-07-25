//! Adapters that project world systems into emulator-facing interfaces.
//!
//! Each adapter is a thin translation surface. Authority stays in the world.

mod memory;
mod bus;
mod interrupt;
mod clock;
mod power;
mod storage;
mod input;
mod display;

pub use memory::MemoryAdapter;
pub use bus::BusAdapter;
pub use interrupt::InterruptAdapter;
pub use clock::ClockAdapter;
pub use power::PowerAdapter;
pub use storage::StorageAdapter;
pub use input::InputAdapter;
pub use display::DisplayAdapter;
