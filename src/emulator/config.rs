//! Minimal machine configuration for early boot experiments.
//!
//! Describes what the integration layer exposes to QEMU. Additional
//! devices can be enabled later without changing world systems.

use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct MinimalMachineConfig {
    /// Guest architecture string for QEMU (`x86_64`).
    pub arch: String,
    /// QEMU machine type — microvm is small and suitable for experiments.
    pub machine: String,
    /// RAM size reported to the guest (bytes). Backed by world RAM regions.
    pub ram_bytes: u64,
    /// Path to a kernel image for -kernel experiments (optional).
    pub kernel_path: Option<String>,
    /// Path to an initrd (optional).
    pub initrd_path: Option<String>,
    /// Disk image path for storage backend experiments (optional).
    pub disk_path: Option<String>,
    /// Enable virtio-keyboard / input forwarding stubs.
    pub enable_input: bool,
    /// Enable display surface forwarding stubs.
    pub enable_display: bool,
    /// QEMU binary name or absolute path.
    pub qemu_binary: String,
    /// Extra QEMU args for advanced experiments.
    pub extra_args: Vec<String>,
}

impl Default for MinimalMachineConfig {
    fn default() -> Self {
        Self {
            arch: "x86_64".into(),
            machine: "microvm".into(),
            ram_bytes: 16 * 1024 * 1024,
            kernel_path: None,
            initrd_path: None,
            disk_path: None,
            enable_input: true,
            enable_display: false,
            qemu_binary: "qemu-system-x86_64".into(),
            extra_args: vec![],
        }
    }
}
