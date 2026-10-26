# Boot ROM

> [!NOTE]
> This page is currently a stub and needs more detail. Several parts of the boot ROM have not been decided yet, and are listed at the bottom of this page.

Every time the console starts, it runs the boot ROM before the game. The boot ROM is not optional, and it is not built into the emulator. It is a separate file that the emulator loads into its own region of memory, and if that file is missing the emulator will not start.

A reset will not run the boot ROM again. The intro animation can be skipped by pressing any binary input on any of the 8 controllers after the heafer is checked.

## Region

The boot ROM lives at `0x430000`, in a slot of 64 KB (see [Memory Map](./memory-map.md)). Unlike the [game's ROM](./rom-layout.md), which is really RAM, the boot ROM is truly read-only and writes to it are ignored. A game that corrupts memory can never stop the console from starting again.

## Boot Sequence

1. The emulator loads the boot ROM file into the boot ROM region, and copies the game file into the ROM region.
2. RAM, VRAM and I/O start out zeroed.
3. The CPU starts running the boot ROM.
4. The boot ROM plays the intro and checks the game's [header](./rom-layout.md): the magic number and the format version. If either is wrong, it shows an error instead of starting the game.
5. The boot ROM puts the console into a clean state.
6. The boot ROM starts the game by jumping to the game's reset vector, using [`JumpIndirect`](./instruction-set.md#indirect-jump-and-call) on the reset vector at `rom start + 0x10`.

## Not Decided Yet

- The format of the boot ROM file, and how the CPU finds its entry point.
- Exactly what state the console is in when a game starts, which step 5 has to produce.
- What the error screen looks like, and what the boot ROM does after showing it.
- Where the emulator looks for the boot ROM file.
