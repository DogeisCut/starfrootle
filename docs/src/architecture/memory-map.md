# Memory Map

> [!NOTE]
> This page is currently a stub and needs more detail.

> [!WARNING]
> Starfrootle is still being built, these regions are very prone to moving, being resized, or having things slotted between them.

Addresses are 3 bytes long and big-endian, giving a 16 MB address space.

Each region ends right before the next one starts. In the case of ROM, it ends at `0xFFFFFF`, the end of address space.

| Start      | Region | Notes                                                                     |
|------------|--------|---------------------------------------------------------------------------|
| `0x000000` | RAM    |                                                                           |
| `0x400000` | VRAM   | See [VRAM Layout](../graphics/vram-layout.md)                             |
| `0x420000` | I/O    | See [I/O Layout](../io/io-layout.md)                                      |
| `0x800000` | ROM    | See [ROM Layout](./rom-layout.md)                                         |

> [!IMPORTANT]
> "ROM" is actually more RAM space, the ROM data from the game is copied to this section during the boot sequence by the hardware.
> This means ROM is actually writable like RAM, because it *is* RAM. Developers are encouraged to only create self-modifying code,
> if they know what they are doing. The mapping seperation exists to keep developers aware.