//! Virtual hardware that exists entirely inside the physics world.
//!
//! Core rule: the physics engine is the single source of truth.
//! Every hardware component is an independent entity that exists because
//! the physics simulation contains it. No external system creates, owns,
//! or simulates these components.
//!
//! The future Linux emulator (and any other software) may only interact
//! with this hardware through the explicit interfaces the hardware itself
//! exposes (buses, MMIO, interrupt lines, etc.).

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// ---------------------------------------------------------------------------
// Common identity for any piece of virtual hardware
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct HardwareComponent {
    pub name: String,
    pub status: ComponentStatus,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ComponentStatus {
    PoweredOff,
    Idle,
    Active,
    Error,
}

// ---------------------------------------------------------------------------
// Concrete hardware components
// Each is an independent entity that lives inside the physics simulation.
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct ComputerCase;

#[derive(Component)]
pub struct Motherboard;

#[derive(Component)]
pub struct Cpu;

#[derive(Component)]
pub struct Ram;

#[derive(Component)]
pub struct Gpu;

#[derive(Component)]
pub struct Storage;

#[derive(Component)]
pub struct Monitor;

#[derive(Component)]
pub struct Keyboard;

#[derive(Component)]
pub struct Mouse;

#[derive(Component)]
pub struct PowerButton;

#[derive(Component)]
pub struct PowerSupply;

// Future devices (network cards, additional storage, sensors, …) follow
// the same pattern: independent entities inside the physics world.

// ---------------------------------------------------------------------------
// Interfaces exposed by hardware
// These are the only channels through which software may interact with
// the hardware. They themselves exist as components inside the same world.
// ---------------------------------------------------------------------------

/// Shared data / power channel.
#[derive(Component)]
pub struct VirtualBus {
    pub data: Vec<u8>,
    pub address: u64,
}

/// Memory-mapped register space that hardware publishes.
/// Software reads and writes here; it never reaches into the hardware entity itself.
#[derive(Component)]
pub struct MemoryMappedIo {
    pub base_address: u64,
    pub size: u64,
}

/// Interrupt line that hardware can raise.
#[derive(Component)]
pub struct InterruptLine {
    pub pending: bool,
    pub vector: u8,
}

/// Marker: this entity participates in the interface layer.
#[derive(Component)]
pub struct BusParticipant;

/// Common communication contract (no ownership transfer).
pub trait BusCommunicator {
    fn read(&self, address: u64, len: usize) -> Option<Vec<u8>>;
    fn write(&mut self, address: u64, data: &[u8]) -> bool;
}

// ---------------------------------------------------------------------------
// Spawning
// All hardware is born inside the physics world as independent entities.
// ---------------------------------------------------------------------------

pub fn spawn_virtual_computer(mut commands: Commands) {
    // Example: the case itself is a physics object.
    // Additional components (motherboard, CPU, …) will be spawned the same way.
    commands.spawn((
        Name::new("ComputerCase"),
        ComputerCase,
        HardwareComponent {
            name: "Case".into(),
            status: ComponentStatus::PoweredOff,
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.4, 0.6, 0.25),
        Transform::from_xyz(0.0, 1.0, 0.0),
        BusParticipant,
    ));

    // TODO: spawn Motherboard, Cpu, Ram, Gpu, Storage, Monitor,
    //       Keyboard, Mouse, PowerButton, PowerSupply, cables, etc.
    //       Each as its own independent physics entity that exposes
    //       the interfaces it needs.

    println!("Virtual computer exists as independent entities inside the physics world.");
    println!("Software will interact with it only through the interfaces those entities expose.");
}
