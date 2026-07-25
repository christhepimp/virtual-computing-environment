# TODO

## Done

- [x] World systems & auto-registration
- [x] Bus transactions, RAM, block storage
- [x] Firmware POST / discovery
- [x] Full power lifecycle (button → PSU → rails → PowerGood)
- [x] Per-device state machines (Offline → Ready / failure on power loss)
- [x] ClockEnable gated by firmware + device progress
- [x] Motherboard bus active only when Ready
- [x] CPU runs only when Ready + firmware Ready

## Next

- [ ] Interactive power button (input / physics interaction)
- [ ] MMIO command handlers for storage & GPU controllers
- [ ] Interrupt delivery into CPU
- [ ] Reset vector / firmware hand-off for guest software
- [ ] Disconnect / entity removal reactions beyond power loss

## Later

- [ ] Linux emulator stub using only hardware interfaces
