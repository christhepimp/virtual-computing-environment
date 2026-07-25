# Architecture

## Core Principle

**The physics engine defines the entire reality of the virtual world.**

Every object that exists inside that world exists because of the physics engine and is part of its simulation. There is no parallel system, no external simulation layer, and no ownership hierarchy that sits outside the physics world.

The physics engine is the single source of truth.

---

## What this means in practice

### 1. The virtual computer is an object *inside* the physics world

The computer (case, motherboard, CPU, RAM, GPU, storage, monitor, keyboard, mouse, power supply, cables, buses, and any future devices) is not a separate system that is “attached” to the physics engine.  
It is a collection of independent entities that exist entirely within the physics simulation. Each component has physical presence (position, mass, collision shape, joints, forces) and follows the same rules as every other object in the room.

### 2. The Linux emulator is also part of that world

The future emulator does not create, own, or simulate the hardware.  
It runs as software *on* the virtual computer. It exists because the virtual hardware exists and is powered. It interacts with that hardware only through the interfaces the hardware itself exposes (buses, memory-mapped registers, interrupt lines, etc.).

The emulator is therefore a process that lives inside the same reality defined by the physics engine. It never steps outside that reality.

### 3. Hardware remains independent

Every hardware component is an independent entity in the physics simulation.  
Nothing else (including the emulator) owns it, parents it, or controls its lifetime. Components can be added, removed, replaced, or physically manipulated at any time. The only coupling is through explicit communication interfaces that the components themselves provide.

### 4. Single source of truth

All subsystems — environment, hardware, interfaces, and future software layers — operate *within* the physics world rather than beside it or above it. Any new research or experimental system must be designed to inhabit this same reality.

---

## Conceptual model

```
┌──────────────────────────────────────────────────────────────────┐
│                    PHYSICS WORLD (Rapier3D)                      │
│                 = the entire virtual reality                     │
│                                                                  │
│   ┌──────────────────────────────────────────────────────────┐   │
│   │  Environment                                             │   │
│   │  (room, lights, atmosphere, props, cables, dust, …)      │   │
│   └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│   ┌──────────────────────────────────────────────────────────┐   │
│   │  Virtual Computer (collection of independent entities)   │   │
│   │                                                          │   │
│   │   Case · Motherboard · CPU · RAM · GPU · Storage         │   │
│   │   Monitor · Keyboard · Mouse · PSU · Buses · …           │   │
│   │                                                          │   │
│   │   Each entity has:                                       │   │
│   │     • physical presence (RigidBody, Collider, Transform) │   │
│   │     • internal state                                     │   │
│   │     • exposed interfaces (bus, MMIO, interrupt lines)    │   │
│   └──────────────────────────────────────────────────────────┘   │
│                          ▲                                       │
│                          │ interfaces only                       │
│   ┌──────────────────────┴───────────────────────────────────┐   │
│   │  Software processes running on the virtual computer      │   │
│   │  (future Linux emulator and any other guest software)    │   │
│   │                                                          │   │
│   │  • do not own hardware                                   │   │
│   │  • do not simulate hardware                              │   │
│   │  • only observe and communicate through the interfaces   │   │
│   │    the hardware itself provides                          │   │
│   └──────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Design rules derived from the core principle

1. **Nothing exists outside the physics world.**  
   Every tangible object and every process that claims to run “on” the computer must be representable as part of the physics simulation (or as software that only talks to interfaces that exist inside it).

2. **Hardware is never owned by software.**  
   The emulator (and any future guest OS or research tool) may only read from and write to the public interfaces that hardware components expose. It may never create, destroy, or take exclusive ownership of a hardware entity.

3. **Interfaces are first-class and replaceable.**  
   Buses, memory-mapped regions, interrupt lines, power rails, and future device protocols are themselves entities or components that live in the same world. They can be swapped or extended independently.

4. **Modularity is mandatory.**  
   Every subsystem (a single chip, a bus protocol, a lighting model, a future research experiment) must be independently replaceable without breaking the rest of the world.

5. **Experimentation is a first-class goal.**  
   The architecture deliberately keeps the physics world open so that new physical objects, new interface protocols, and new software layers can be introduced and studied without rewriting the foundation.

---

## Module layout (current)

- `physics` – the root reality; configuration and any world-level rules
- `environment` – the room and atmospheric elements that also live inside the physics world
- `hardware` – independent component entities + the interface components they expose
- `emulator` – placeholder for the future software process that will run *on* the virtual computer through those interfaces

All of the above are subsystems that inhabit the single physics world; none of them sit outside it.

---

## Future research directions supported by this model

- Physical disassembly and reassembly of the virtual computer
- Novel bus protocols or interconnects introduced as new entities
- Multiple competing “guest” software stacks running on the same hardware
- Direct manipulation of power, heat, or signal integrity as physical phenomena
- Experiments that treat the emulator itself as an observable process inside the room

Everything remains inside one consistent reality defined by the physics engine.
