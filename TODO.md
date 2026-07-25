# TODO

## Done

- [x] World systems + hardware lifecycle
- [x] Emulator integration layer + QEMU transport
- [x] Real memory path through MemoryMapSystem
- [x] Real storage path through StorageSystem + virtual disk signature
- [x] IRQ raise/ack path through InterruptSystem
- [x] Clock adapter from ClockSystem
- [x] Boot experiment runner + asset discovery + logging

## Next

- [ ] Bidirectional IPC (shared memory / socket) for live QEMU RAM ops
- [ ] Virtio-blk backend wired to StorageSystem
- [ ] Capture QEMU serial into boot log
- [ ] Reset vector / firmware hand-off address for guest entry

## Later

- [ ] Full kernel boot CI fixture with buildroot image
