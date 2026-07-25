# TODO

## Done

- [x] World systems, registration, buses, memory, storage
- [x] Firmware + full power / device lifecycle
- [x] Emulator Integration Layer (adapters + QEMU transport)
- [x] Dry-run mode when QEMU binary is absent
- [x] Minimal machine config for boot experiments

## Next

- [ ] Shared-memory or socket IPC between QEMU and MemoryAdapter
- [ ] Wire virtio-blk requests into StorageSystem
- [ ] IRQCHIP-style delivery from InterruptSystem into guest
- [ ] Optional -kernel boot experiment path end-to-end
- [ ] Interactive power button

## Later

- [ ] Richer device set without moving authority out of the world
- [ ] Display path from GPU controller to DisplayAdapter
