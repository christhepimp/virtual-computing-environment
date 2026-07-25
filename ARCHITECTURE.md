# Project Architecture

## Core Principles

1. **Physics engine is the master world**  
   Rapier3D owns the simulation. Every tangible object (room geometry, computer case, motherboard, cables, etc.) is a physics entity that obeys collisions, gravity, joints, and forces.

2. **Hardware is independent**  
   Each hardware component (Case, Motherboard, CPU, RAM, GPU, Storage, Monitor, Keyboard, Mouse, Power Button, …) is a first-class entity in the physics world.  
   The emulator does **not** own, parent, or control the lifetime of any hardware entity.

3. **Emulator communicates only through interfaces / buses**  
   The sole interaction path between the future Linux emulator and the hardware is a set of pure communication components:
   - `VirtualBus` – shared data / power channel
   - `MemoryMappedIo` – register space the emulator can read/write
   - `InterruptLine` – events the hardware can raise and the emulator observes

   The emulator is a pure software process (no physics body) that queries and mutates only these interface components.

4. **Modularity & replaceability**  
   Any hardware component can be swapped, removed, or replaced at runtime because nothing else holds ownership of it.

## Layers

```
┌─────────────────────────────────────────────────────┐
│  Physics Layer (Rapier3D)                           │
│  – owns the world, gravity, collisions, joints      │
└─────────────────────────────────────────────────────┘
          ▲
          │ every tangible object lives here
┌─────────┴───────────────────────────────────────────┐
│  Environment Layer                                  │
│  – room, lighting, atmosphere, interactive props    │
└─────────────────────────────────────────────────────┘
          ▲
┌─────────┴───────────────────────────────────────────┐
│  Hardware Layer                                     │
│  – independent entities: Case, MB, CPU, RAM, …      │
│  – each carries RigidBody / Collider + status       │
└─────────────────────────────────────────────────────┘
          ▲
          │ communicate exclusively via
┌─────────┴───────────────────────────────────────────┐
│  Interface / Bus Layer                              │
│  – VirtualBus, MemoryMappedIo, InterruptLine        │
└─────────────────────────────────────────────────────┘
          ▲
┌─────────┴───────────────────────────────────────────┐
│  Emulator Layer (future)                            │
│  – pure software process                            │
│  – NEVER owns hardware                              │
│  – only reads/writes the interface components       │
└─────────────────────────────────────────────────────┘
```

## Extensibility

- Bevy ECS makes every piece a component that can be queried independently.
- New hardware is added by spawning a new entity with the appropriate marker + physics + bus participation.
- The emulator plugin will later register systems that only touch the bus / MMIO / interrupt queries.

This separation guarantees that the virtual computer remains a physical object you can walk around, pick up, disassemble, and experiment with, while the software stack (emulator) remains a clean, non-owning client of the hardware interfaces.
