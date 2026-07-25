use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Hardware components are independent entities that live in the physics world.
/// They are never owned by the emulator. The emulator only talks to them through
/// explicit bus / interface components and systems.

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
// Individual hardware components (each is its own physics entity)
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

// ---------------------------------------------------------------------------
// Bus / Interface layer – the ONLY way the emulator (or any other system)
// may interact with hardware.
// ---------------------------------------------------------------------------

/// A shared data / power bus that hardware components and the emulator
/// can read from / write to. This is a pure communication channel.
#[derive(Component)]
pub struct VirtualBus {
    pub data: Vec<u8>,
    pub address: u64,
}

/// Memory-mapped I/O region. Hardware exposes registers here;
/// the emulator reads/writes them without ever owning the hardware entity.
#[derive(Component)]
pub struct MemoryMappedIo {
    pub base_address: u64,
    pub size: u64,
}

/// Interrupt line that hardware can raise. Emulator observes these events.
#[derive(Component)]
pub struct InterruptLine {
    pub pending: bool,
    pub vector: u8,
}

/// Marker for any entity that participates in the hardware interface layer.
#[derive(Component)]
pub struct BusParticipant;

// ---------------------------------------------------------------------------
// Trait for components that can talk on a bus (implemented by both hardware
// and the future emulator side). No ownership transfer occurs.
// ---------------------------------------------------------------------------

pub trait BusCommunicator {
    fn read(&self, address: u64, len: usize) -> Option<Vec<u8>>;
    fn write(&mut self, address: u64, data: &[u8]) -> bool;
}

// ---------------------------------------------------------------------------
// Spawning – all hardware is created as independent physics entities.
// ---------------------------------------------------------------------------

pub fn spawn_virtual_computer(mut commands: Commands) {
    // Each component is an independent entity with its own RigidBody / Collider.
    // They are linked only by bus / interface components, never by ownership.

    // Example skeleton (meshes & full colliders will be added later):
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

    // Motherboard, CPU, RAM, GPU, Storage, Monitor, Keyboard, Mouse,
    // PowerButton would be spawned the same way – each as its own entity.

    println!("Virtual computer hardware spawned as independent physics entities.");
    println!("Emulator will communicate only through VirtualBus / MemoryMappedIo / InterruptLine.");
}
