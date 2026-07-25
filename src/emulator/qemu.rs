//! QEMU transport.
//!
//! Starts an external QEMU process for boot experiments when available.
//! When QEMU is not installed, operates in DryRun mode so adapters and
//! lifecycle still exercise the integration layer against world systems.
//!
//! Important: QEMU is treated as guest *software execution* infrastructure.
//! Memory, IRQs, storage, and power continue to be served from world systems
//! through the integration layer — QEMU is not granted ownership of the
//! virtual motherboard, RAM entities, or device lifecycle.

use std::path::Path;
use std::process::Command;

use super::config::MinimalMachineConfig;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum QemuTransportState {
    #[default]
    Idle,
    Starting,
    Running,
    DryRun,
    Stopped,
}

#[derive(Clone, Debug)]
pub enum GuestRequest {
    MemoryRead { addr: u64, len: usize },
    MemoryWrite { addr: u64, data: Vec<u8> },
    StorageRead { entity_bits: u64, lba: u64, count: u64 },
    StorageWrite { entity_bits: u64, lba: u64, data: Vec<u8> },
    InterruptAck { vector: u8 },
}

#[derive(Default, Debug)]
pub struct QemuTransport {
    pub state: QemuTransportState,
    pub pid: Option<u32>,
    pending_requests: Vec<GuestRequest>,
    pub last_mem_read: Option<(u64, Option<Vec<u8>>)>,
    pub last_storage_read: Option<(u64, Option<Vec<u8>>)>,
    pub injected_keys: Vec<u8>,
    pub injected_mouse: Vec<[u8; 3]>,
    pub pending_irqs: Vec<u8>,
}

impl QemuTransport {
    pub fn mode_name(&self) -> &'static str {
        match self.state {
            QemuTransportState::DryRun => "dry-run",
            QemuTransportState::Running => "qemu-process",
            QemuTransportState::Starting => "starting",
            QemuTransportState::Stopped => "stopped",
            QemuTransportState::Idle => "idle",
        }
    }

    pub fn start(&mut self, config: &MinimalMachineConfig) -> Result<(), String> {
        self.state = QemuTransportState::Starting;

        let bin = &config.qemu_binary;
        let available = Command::new(bin)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if !available {
            self.state = QemuTransportState::DryRun;
            return Err(format!("{bin} not available on PATH"));
        }

        let mut cmd = Command::new(bin);
        cmd.arg("-machine").arg(&config.machine);

        let mem_mb = (config.ram_bytes / (1024 * 1024)).max(16);
        cmd.arg("-m").arg(format!("{mem_mb}M"));
        cmd.arg("-nographic");
        cmd.arg("-no-reboot");
        cmd.arg("-nodefaults");
        cmd.arg("-serial").arg("stdio");

        if let Some(kernel) = &config.kernel_path {
            if Path::new(kernel).exists() {
                cmd.arg("-kernel").arg(kernel);
            }
        }
        if let Some(initrd) = &config.initrd_path {
            if Path::new(initrd).exists() {
                cmd.arg("-initrd").arg(initrd);
            }
        }
        if let Some(disk) = &config.disk_path {
            if Path::new(disk).exists() {
                cmd.arg("-drive")
                    .arg(format!("file={disk},format=raw,if=virtio"));
            }
        }

        for arg in &config.extra_args {
            cmd.arg(arg);
        }

        match cmd.spawn() {
            Ok(child) => {
                self.pid = Some(child.id());
                let _ = child;
                self.state = QemuTransportState::Running;
                println!("[QemuTransport] spawned pid={:?}", self.pid);
                Ok(())
            }
            Err(e) => {
                self.state = QemuTransportState::DryRun;
                Err(e.to_string())
            }
        }
    }

    pub fn stop(&mut self) {
        self.state = QemuTransportState::Stopped;
        self.pid = None;
        self.pending_requests.clear();
    }

    pub fn notify_interrupt(&mut self, vector: u8) {
        self.pending_irqs.push(vector);
    }

    pub fn inject_key(&mut self, scancode: u8) {
        self.injected_keys.push(scancode);
    }

    pub fn inject_mouse(&mut self, packet: [u8; 3]) {
        self.injected_mouse.push(packet);
    }

    pub fn poll_requests(&mut self) -> Vec<GuestRequest> {
        std::mem::take(&mut self.pending_requests)
    }

    pub fn enqueue_request(&mut self, req: GuestRequest) {
        self.pending_requests.push(req);
    }

    pub fn complete_memory_read(&mut self, addr: u64, data: Option<Vec<u8>>) {
        self.last_mem_read = Some((addr, data));
    }

    pub fn complete_storage_read(&mut self, entity_bits: u64, data: Option<Vec<u8>>) {
        self.last_storage_read = Some((entity_bits, data));
    }
}
