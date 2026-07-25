# TODO

## Done

- [x] World systems (power, clock, registry, buses, signals, memory, interrupts, connections)
- [x] Auto-registration of hardware entities
- [x] Bus transactions (read/write cycles)
- [x] RAM backing store + memory map
- [x] Block storage interface
- [x] Firmware POST / discovery / init sequence
- [x] Motherboard bus routing
- [x] CPU issues memory reads via bus
- [x] Device controllers (keyboard, mouse, GPU, storage)

## Next

- [ ] Power button interaction toggles PowerSystem
- [ ] Richer MMIO handlers for storage/GPU/keyboard controllers
- [ ] Interrupt delivery into CPU core
- [ ] Firmware hand-off vector / reset vector for guest software
- [ ] Room geometry inside the physics world

## Later

- [ ] Non-owning Linux emulator stub using only hardware interfaces
- [ ] Multiple CPUs / APIC-style interrupt routing
- [ ] DMA-style transfers through the bus system
