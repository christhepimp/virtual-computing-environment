# Boot assets for Linux experiments

Place files here for a real QEMU-backed boot attempt:

| File | Purpose |
|------|--------|
| `vmlinuz` or `bzImage` | Linux kernel |
| `initrd.img` or `initramfs.cpio` | Optional initramfs |
| `rootfs.img` | Optional raw disk image |

Example (host):

```bash
mkdir -p assets/boot
# copy a small kernel built for x86_64, e.g. from a distro or buildroot
cp /path/to/bzImage assets/boot/
cp /path/to/initramfs.cpio assets/boot/initrd.img
```

Then run:

```bash
cargo run
```

Without these files the integration layer still runs in **dry-run** mode and exercises memory, storage, IRQ, and clock paths through the world systems.
