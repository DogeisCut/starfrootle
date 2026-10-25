# Memory Map

> [!NOTE]
> This page is currently a stub and needs more detail.

Addresses are 3 bytes long and big-endian, giving a 16 MB address space.

| Start | Region | Notes |
|---|---|---|
| `0x000000` | RAM | |
| `0x400000` | VRAM | See [VRAM Layout](../graphics/vram-layout.md) |
| `0x800000` | ROM | Copied in from whatever game is loaded |