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

## World systems (active runtime)

The physics world hosts a set of **world systems** that manage the entire state of the virtual computer. These systems are the single source of truth for hardware state. Hardware entities do not maintain isolated authoritative state outside them.

| System | Responsibility |
|--------|----------------|
| **PowerSystem** | Main power rail, per-device power, consumption |
| **ClockSystem** | Master clock and per-device clock domains |
| **DeviceRegistry** | Discovery & registration of every hardware entity |
| **BusSystem** | Virtual buses and traffic |
| **SignalSystem** | Discrete signals (reset, power-good, …) |
| **MemoryMapSystem** | Address space → device mapping |
| **InterruptSystem** | Pending interrupt vectors |
| **ConnectionSystem** | Topology (socket, cable, bus, power rail) |

### Registration model

1. A hardware entity is spawned in the physics world with a `RegisterDevice` component (and optional `PoweredDevice`, `ClockedDevice`, `BusAttachment`, `MemoryMappedRegion`, `InterruptSource`, …).
2. The world systems discover it and enroll it automatically.
3. Authoritative state for that device lives in the world systems.
4. Removing the entity triggers unregistration.

New hardware is added by creating a new entity and letting it register. No existing architecture needs to change.

### Emulator relationship

The future Linux emulator will run as software inside this virtual computer. It will only communicate through the world systems and the interfaces published by hardware. It will never own, create, or simulate hardware.

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
│   │  World Systems (single source of truth for machine state)│   │
│   │  Power · Clock · Registry · Buses · Signals              │   │
│   │  Memory Map · Interrupts · Connections                   │   │
│   └──────────────────────────────────────────────────────────┘   │
│                          ▲                                       │
│                          │ auto-registration + interfaces        │
│   ┌──────────────────────┴───────────────────────────────────┐   │
│   │  Virtual Computer (independent entities)                 │   │
│   │  Case · Motherboard · CPU · RAM · GPU · Storage          │   │
│   │  Monitor · Keyboard · Mouse · PSU · …                    │   │
│   └──────────────────────────────────────────────────────────┘   │
│                          ▲                                       │
│                          │ world systems & interfaces only       │
│   ┌──────────────────────┴───────────────────────────────────┐   │
│   │  Software processes (future Linux emulator, …)           │   │
│   │  • do not own hardware                                   │   │
│   │  • do not simulate hardware                              │   │
│   │  • only communicate through world systems / interfaces   │   │
│   └──────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Design rules

1. **Nothing exists outside the physics world.**
2. **Hardware is never owned by software.**
3. **World systems own authoritative machine state.**
4. **Interfaces and registration are how entities join the machine.**
5. **Modularity is mandatory** — new hardware = new entity + registration.
6. **Experimentation is a first-class goal.**

---

## Module layout

- `physics` — root reality
- `environment` — room and atmosphere (inside the world)
- `world` — power, clock, devices, buses, signals, memory, interrupts, connections
- `hardware` — independent component entities that register with world systems
- `emulator` — non-owning placeholder for future guest software
