# First Linux Boot Experiment

## Data path (unchanged architecture)

```
Physics World
  → Virtual Hardware (entities + lifecycle)
    → Hardware Interfaces (buses, MMIO, IRQ, power, clock, storage)
      → Emulator Integration Layer (adapters)
        → QemuTransport (process or dry-run)
          → Linux guest
```

QEMU does not own RAM, disks, IRQs, or power. Those remain in world systems.

## What this milestone implements

1. **Memory** — `GuestRequest::MemoryRead/Write` → `MemoryAdapter` → `MemoryMapSystem`
2. **Storage** — block R/W → `StorageSystem`; LBA0 signature for virtual disk
3. **Interrupts** — devices raise via `InterruptSystem`; adapter notifies transport
4. **Clock** — `ClockAdapter` mirrors authoritative `ClockSystem`
5. **Boot experiment** — waits for firmware Ready, loads `assets/boot/*`, starts transport
6. **Logging** — power, devices, memory, storage, IRQ, clock, boot phase snapshots

## Running

```bash
# Optional real kernel
mkdir -p assets/boot
# add vmlinuz/bzImage and optional initrd

cargo run
```

Watch stdout for `[Boot]`, `[Path][Memory]`, `[Path][Storage]`, `[Path][IRQ]`, `[Path][Clock]`, `[Path][Power]`.

## Success criteria

- **Dry-run success**: machine reaches Ready; memory/storage/IRQ/clock paths log through world systems.
- **Real QEMU success**: `qemu-system-x86_64` on PATH + kernel in `assets/boot/` → transport Running; serial shows kernel messages (host terminal).

## Non-goals

- Desktop environment
- QEMU as hardware authority
- Redesign of world systems
