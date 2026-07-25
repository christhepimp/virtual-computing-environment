# Virtual Computing Environment

A 3D virtual world whose entire reality is defined by an open-source physics engine.

The virtual computer is not a separate system attached to the simulation.  
It is a collection of independent physical objects that exist *inside* the physics world.  
Future software (including a Linux emulator) runs on that computer and interacts with it only through the interfaces the hardware itself exposes.

There is a single source of truth: the physics simulation.

## Core Principle

> The physics engine defines the entire reality of the virtual world.  
> Every object that exists inside that world exists because of the physics engine and is part of its simulation.

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full design rules and conceptual model.

## Current Status (Foundation)

- Physics world (Rapier3D) is the root reality
- Modular hardware entities (case, and placeholders for the rest)
- Explicit interface layer (buses, MMIO, interrupts) for software interaction
- Emulator module present only as a non-owning placeholder
- Eerie room atmosphere scaffolding

No full Linux emulator is implemented yet. The architecture is deliberately prepared for it while keeping hardware independent and the physics world as the sole source of truth.

## Tech Stack

- Rust
- Bevy (ECS, rendering, input)
- Rapier3D (physics)

## Getting Started

```bash
git clone https://github.com/christhepimp/virtual-computing-environment.git
cd virtual-computing-environment
cargo run
```

## Goals

- Experimentation and research inside a consistent physical reality
- Fully modular, independently replaceable subsystems
- Long-term path toward a guest operating system that truly runs on virtual hardware that itself lives in the physics world
