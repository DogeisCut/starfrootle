# I/O Layout

> [!NOTE]
> This page is currently a stub and needs more detail.

Where I/O starts is defined in the [memory map](../architecture/memory-map.md). Each device gets a slot of `0x1000` bytes, so a device can grow without the others having to move.

| Offset    | Size    | Contents                              |
|-----------|---------|---------------------------------------|
| `+0x0000` | `0x80`  | [Controller table](#controller-table) |
| `+0x1000` | `0x400` | [Audio channel table](#audio)         |
| `+0x1400` | `0x10`  | [Global audio block](#audio)          |

## Controller Table 

8 [controller entries](./input.md#controller-entry) of 16 bytes each, for a total of 128 bytes. Controller N lives 
at `i/o start + N * 16`.


## Audio

16 [audio channel entries](./audio.md#channel-entry) of 64 bytes each, for a total of 1 KB. Channel N lives at `i/o start + 0x1000 + N * 0x40`.

The [global audio block](./audio.md#global-block) is 16 bytes and lives right after the channel table, at `i/o start + 0x1400`.

> [!NOTE]
> Below this point are planned features. None of these have been expanded on yet.

## System Status
- Vblank flag
- Current scanline
- Free-running frame counter

## Interrupt control
- Enable and status bits for vblank and scanline.
- A scanline compare value.

## Save memory
- A small region of RAM is dedicated to save data, and is kept between sessions. Its place in memory is in the [Memory Map](../architecture/memory-map.md).
- Game ID used in the header, emulator uses the game id to help name the save file. 
- Emulator writes the save data to a file.

## Debug port
- A byte you write to prints a character in the emulator’s terminal, plus an assert or breakpoint register.

## RNG
- A register that returns the next value of a seeded generator, with a writable seed.

## Timers
- Countdowns that aren’t tied to frames.

## Real-time clock
- Host date and time.