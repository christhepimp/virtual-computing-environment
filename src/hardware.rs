use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Modular hardware components
#[derive(Component)]
pub struct HardwareComponent {
    pub name: String,
    pub status: ComponentStatus,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ComponentStatus {
    PoweredOff,
    Idle,
    Active,
    Error,
}

// Hardware interface layer
pub trait HardwareInterface {
    fn connect(&mut self, other: &mut dyn HardwareInterface);
    fn send_data(&self, data: Vec<u8>);
    // Placeholder for bus communication
}

// Specific components - placeholders
#[derive(Component)]
pub struct Motherboard;

#[derive(Component)]
pub struct Cpu;

// etc. for RAM, GPU, etc.

pub fn spawn_virtual_computer(mut commands: Commands) {
    // Spawn case, then internal components as children or linked via interfaces
    println!("Virtual computer hardware spawned with modular components.");
    // Future: Each component has physics body, can be manipulated, powered on/off
}

// Placeholder for Linux emulator connection point
pub struct EmulatorInterface;
// The emulator will interface here with the hardware layer
