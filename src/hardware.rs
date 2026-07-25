//! Virtual hardware entities that exist inside the physics world.
//!
//! Each component is an independent entity. On spawn it carries a
//! `RegisterDevice` component so the world systems automatically
//! discover and enroll it. Authoritative state lives in the world
//! systems (power, clock, registry, buses, …), not in isolated
//! per-component fields.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::world::devices::{DeviceKind, RegisterDevice};
use crate::world::power::PoweredDevice;
use crate::world::clock::ClockedDevice;
use crate::world::buses::{BusAttachment, BusId};
use crate::world::memory::MemoryMappedRegion;
use crate::world::interrupts::InterruptSource;

// ---------------------------------------------------------------------------
// Hardware identity markers (independent physics entities)
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

// ---------------------------------------------------------------------------
// Spawning the virtual computer as independent entities inside the world
// ---------------------------------------------------------------------------

pub fn spawn_virtual_computer(mut commands: Commands) {
    // Case
    commands.spawn((
        Name::new("ComputerCase"),
        ComputerCase,
        RegisterDevice {
            name: "Case".into(),
            kind: DeviceKind::Case,
        },
        PoweredDevice { wattage: 5.0 },
        RigidBody::Dynamic,
        Collider::cuboid(0.4, 0.6, 0.25),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    // Motherboard
    commands.spawn((
        Name::new("Motherboard"),
        Motherboard,
        RegisterDevice {
            name: "Motherboard".into(),
            kind: DeviceKind::Motherboard,
        },
        PoweredDevice { wattage: 15.0 },
        BusAttachment {
            buses: vec![BusId::System, BusId::Memory, BusId::Io],
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.3, 0.02, 0.25),
        Transform::from_xyz(0.0, 0.9, 0.0),
    ));

    // CPU
    commands.spawn((
        Name::new("CPU"),
        Cpu,
        RegisterDevice {
            name: "CPU".into(),
            kind: DeviceKind::Cpu,
        },
        PoweredDevice { wattage: 65.0 },
        ClockedDevice { hz: 3_000_000_000 },
        BusAttachment {
            buses: vec![BusId::System, BusId::Memory],
        },
        InterruptSource { default_vector: 0 },
        MemoryMappedRegion {
            base: 0x0000_0000,
            size: 0x1000,
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.04, 0.01, 0.04),
        Transform::from_xyz(0.0, 0.95, 0.0),
    ));

    // RAM
    commands.spawn((
        Name::new("RAM"),
        Ram,
        RegisterDevice {
            name: "RAM".into(),
            kind: DeviceKind::Ram,
        },
        PoweredDevice { wattage: 8.0 },
        BusAttachment {
            buses: vec![BusId::Memory],
        },
        MemoryMappedRegion {
            base: 0x0010_0000,
            size: 0x1000_0000, // 256 MiB abstract window
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.12, 0.02, 0.03),
        Transform::from_xyz(0.15, 0.95, 0.05),
    ));

    // GPU
    commands.spawn((
        Name::new("GPU"),
        Gpu,
        RegisterDevice {
            name: "GPU".into(),
            kind: DeviceKind::Gpu,
        },
        PoweredDevice { wattage: 150.0 },
        ClockedDevice { hz: 1_500_000_000 },
        BusAttachment {
            buses: vec![BusId::System, BusId::Memory],
        },
        MemoryMappedRegion {
            base: 0xE000_0000,
            size: 0x1000_0000,
        },
        InterruptSource { default_vector: 16 },
        RigidBody::Dynamic,
        Collider::cuboid(0.25, 0.04, 0.12),
        Transform::from_xyz(0.0, 0.7, 0.1),
    ));

    // Storage
    commands.spawn((
        Name::new("Storage"),
        Storage,
        RegisterDevice {
            name: "Storage".into(),
            kind: DeviceKind::Storage,
        },
        PoweredDevice { wattage: 6.0 },
        BusAttachment {
            buses: vec![BusId::Io],
        },
        MemoryMappedRegion {
            base: 0xF000_0000,
            size: 0x1000,
        },
        InterruptSource { default_vector: 14 },
        RigidBody::Dynamic,
        Collider::cuboid(0.1, 0.02, 0.07),
        Transform::from_xyz(-0.15, 0.6, 0.0),
    ));

    // Monitor
    commands.spawn((
        Name::new("Monitor"),
        Monitor,
        RegisterDevice {
            name: "Monitor".into(),
            kind: DeviceKind::Monitor,
        },
        PoweredDevice { wattage: 30.0 },
        RigidBody::Dynamic,
        Collider::cuboid(0.3, 0.25, 0.05),
        Transform::from_xyz(0.0, 1.4, -0.6),
    ));

    // Keyboard
    commands.spawn((
        Name::new("Keyboard"),
        Keyboard,
        RegisterDevice {
            name: "Keyboard".into(),
            kind: DeviceKind::Keyboard,
        },
        PoweredDevice { wattage: 1.0 },
        BusAttachment {
            buses: vec![BusId::Io],
        },
        InterruptSource { default_vector: 1 },
        RigidBody::Dynamic,
        Collider::cuboid(0.22, 0.02, 0.08),
        Transform::from_xyz(0.0, 0.85, 0.5),
    ));

    // Mouse
    commands.spawn((
        Name::new("Mouse"),
        Mouse,
        RegisterDevice {
            name: "Mouse".into(),
            kind: DeviceKind::Mouse,
        },
        PoweredDevice { wattage: 0.5 },
        BusAttachment {
            buses: vec![BusId::Io],
        },
        InterruptSource { default_vector: 12 },
        RigidBody::Dynamic,
        Collider::cuboid(0.03, 0.02, 0.05),
        Transform::from_xyz(0.3, 0.85, 0.5),
    ));

    // Power button
    commands.spawn((
        Name::new("PowerButton"),
        PowerButton,
        RegisterDevice {
            name: "PowerButton".into(),
            kind: DeviceKind::PowerButton,
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.015, 0.015, 0.01),
        Transform::from_xyz(0.35, 1.1, 0.26),
    ));

    // Power supply
    commands.spawn((
        Name::new("PowerSupply"),
        PowerSupply,
        RegisterDevice {
            name: "PowerSupply".into(),
            kind: DeviceKind::PowerSupply,
        },
        PoweredDevice { wattage: 10.0 },
        RigidBody::Dynamic,
        Collider::cuboid(0.15, 0.1, 0.15),
        Transform::from_xyz(0.0, 0.4, -0.1),
    ));

    println!("Virtual computer spawned as independent entities inside the physics world.");
    println!("Each device will auto-register with the world systems.");
}
