# TODO

## Foundation (current focus)

- [ ] Room geometry and atmosphere that live inside the physics world
- [ ] Full set of independent hardware entities (motherboard, CPU, RAM, GPU, storage, monitor, keyboard, mouse, PSU, cables)
- [ ] Each hardware entity exposes the interfaces it needs (bus, MMIO, interrupts, power)
- [ ] Basic interaction (raycast / grab / power button)
- [ ] Minimal power and signal flow through the interface layer

## Next

- [ ] First non-owning emulator stub that only talks to the interfaces
- [ ] Visual and physical feedback when hardware state changes
- [ ] Eerie effects (flicker, dust, subtle audio) as part of the same world

## Long-term research directions

- Physical disassembly and reassembly of the virtual computer
- Novel bus or interconnect experiments introduced as new entities
- Multiple guest software stacks on the same hardware
- Treating heat, power integrity, or signal quality as first-class physical phenomena
- Observing the emulator process itself as something that exists inside the room

All work remains inside the single reality defined by the physics engine.
