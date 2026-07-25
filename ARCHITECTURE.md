# Project Architecture

## Core Principles
- Physics engine is the master world.
- All objects, including virtual hardware, obey physics rules.
- Modular, replaceable components.
- Hardware interface layer for communication.
- Placeholder architecture for future Linux emulator.

## Layers
1. **Physics Layer**: Rapier3D - governs everything.
2. **Environment Layer**: 3D room with interactive elements.
3. **Hardware Layer**: Individual components (Case, MB, CPU, etc.) with physics bodies.
4. **Interface Layer**: Bus/system for data/power flow.
5. **Emulator Layer**: (Future) Runs 'inside' the virtual machine.

## Extensibility
- Use Bevy ECS for components.
- Traits for hardware interfaces.
- Scene loading for complex models.

Future expansions: User interaction with mouse/keyboard in room, powering on components, debugging virtual hardware, etc.