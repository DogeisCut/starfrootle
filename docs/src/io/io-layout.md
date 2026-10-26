# I/O Layout

> [!NOTE]
> This page is currently a stub and needs more detail.

I/O starts at `0x420000`. Each device gets a slot of `0x1000` bytes, so a device can grow without the others having to move.

| Offset  | Size         | Contents                              |
|---------|--------------|---------------------------------------|
| `+0x00` | `0x80`       | [Controller table](#controller-table) |
| `+0x1000` | `0x400`    | [Audio channel table](#audio)         |
| `+0x1400` | `0x10`     | [Global audio block](#audio)          |


## Controller Table 

8 [controller entries](./input.md#controller-entry) of 16 bytes each, for a total of 128 bytes. Controller N lives 
at `i/o start + N * 16`.


## Audio

16 [audio channel entries](./audio.md#channel-entry) of 64 bytes each, for a total of 1 KB. Channel N lives at `i/o start + 0x1000 + N * 0x40`.

The [global audio block](./audio.md#global-block) is 16 bytes and lives right after the channel table, at `i/o start + 0x1400`.
