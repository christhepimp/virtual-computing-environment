# TODO

## Done (world infrastructure)

- [x] Power system (world authority)
- [x] Clock system
- [x] Device discovery & registration
- [x] Virtual buses
- [x] Signals
- [x] Memory mapping
- [x] Interrupts
- [x] Device connections
- [x] Hardware entities auto-register with world systems

## Next foundation steps

- [ ] Room geometry that lives inside the physics world
- [ ] Richer meshes / colliders for each hardware component
- [ ] Power-button interaction → PowerSystem.main_power toggle
- [ ] Wire MemoryMappedRegion components into MemoryMapSystem on registration
- [ ] Wire BusAttachment into BusSystem on registration
- [ ] Basic signal routing (reset, power-good)

## Later

- [ ] First non-owning emulator stub that only talks to world systems
- [ ] Visual / physical feedback for power and activity state
- [ ] Eerie atmosphere effects as part of the same world

## Research directions (still inside the physics world)

- Physical disassembly / reassembly
- New bus or interconnect entities
- Multiple guest software stacks
- Heat / power integrity as simulated phenomena
