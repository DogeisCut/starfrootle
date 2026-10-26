# Memory Map

> [!NOTE]
> This page is currently a stub and needs more detail.

Addresses are 3 bytes long and big-endian, giving a 16 MB address space.

Each region ends right before the next one starts. In the case of ROM, it ends at `0xFFFFFF`, the end of address space.

| Start      | Region | Notes                                                                     |
|------------|--------|---------------------------------------------------------------------------|
| `0x000000` | RAM    |                                                                           |
| `0x400000` | VRAM   | See [VRAM Layout](../graphics/vram-layout.md)                             |
| `0x420000` | I/O    | See [I/O Layout](../io/io_layout.md)                                      |
| `0x800000` | ROM    | Copied in from whatever game is loaded, See [ROM Layout](./rom_layout.md) |