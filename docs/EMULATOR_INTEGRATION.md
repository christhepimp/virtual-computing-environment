# Emulator Integration (QEMU)

## Principle

The physics world remains the single source of truth. The virtual computer is the hardware platform. QEMU is introduced only as **guest software execution** infrastructure. It does **not** own motherboard, RAM entities, device lifecycle, power, or buses.

All authoritative state stays in world systems:

- `PowerSystem`, `ClockSystem`, `DeviceRegistry`
- `BusSystem`, `SignalSystem`
- `MemoryMapSystem`, `InterruptSystem`, `StorageSystem`
- `ConnectionSystem`, firmware, device FSMs

## Integration Layer

```
┌─────────────────────────────────────────────┐
│              Physics World                  │
│  World systems + hardware entities          │
└──────────────────▲──────────────────────────┘
                   │ read / write through
                   │ existing interfaces only
┌──────────────────┴──────────────────────────┐
│         Emulator Integration Layer          │
│  Power · Clock · Memory · IRQ · Storage     │
│  Bus · Input · Display adapters             │
└──────────────────▲──────────────────────────┘
                   │ transport
┌──────────────────┴──────────────────────────┐
│  QEMU transport (process or dry-run)        │
│  — executes guest code                      │
│  — does not own virtual hardware            │
└─────────────────────────────────────────────┘
```

## Adapters

| Adapter | World source | Guest use |
|---------|--------------|-----------|
| Memory | `MemoryMapSystem` | RAM read/write |
| Bus | `BusSystem` | MMIO/PIO transactions |
| Interrupt | `InterruptSystem` | IRQ inject / ACK |
| Clock | `ClockSystem` | timers / timebase |
| Power | `PowerSystem` + signals | power-good / halt |
| Storage | `StorageSystem` | block read/write |
| Input | queues → transport | keyboard/mouse |
| Display | GPU/monitor view | framebuffer (stub) |

## Lifecycle

1. Firmware reaches `Ready` and `PowerGood` is asserted.
2. Integration layer **Arms** then starts transport.
3. If `qemu-system-x86_64` is missing, **dry-run** mode still syncs adapters.
4. Power loss or firmware leave-Ready **halts** the guest transport.

## Minimal configuration

See `MinimalMachineConfig` (`src/emulator/config.rs`):

- arch: `x86_64`
- machine: `microvm`
- RAM: 16 MiB (matches foundation RAM window)
- optional `-kernel` / `-initrd` / disk paths

## Extending hardware

Add a world-side device + registration as today, then extend the corresponding adapter (or add a new one). Do not teach QEMU to own that device.

## Non-goals (this stage)

- Full virtio-pci device models inside QEMU that bypass world systems
- QEMU as the authority for RAM contents or IRQ routing
- Redesign of core world systems
