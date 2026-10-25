# Tile Layers

> [!NOTE]
> This page is currently a stub and needs more detail.

There are 5 tile layers, numbered `0` to `4`. Layer 0 is the topmost. Each layer has its own tile map, and each layer scrolls independently.

## Tile Maps

A tile map is 64x64 [tile entries](./tiles.md) of 4 bytes each, which is 16 KB (`0x4000`) per layer. The screen is 30x30 tiles, so a map is a little more than twice the screen in each direction (512x512 pixels).

Entries are stored one row at a time, so the entry at column X and row Y (both `0` to `63`) is at `map base + Y * 0x100 + X * 4`. The address of each layer's map is in [VRAM Layout](./vram-layout.md).

## Scrolling

Scroll X and scroll Y are each 1 byte (`0` to `255`). A scroll value is the position in the map of the top left corner of the screen, in pixels.

At the largest scroll value the screen shows up to pixel 494 (255 + 239), which is inside tile column or row 61. Columns and rows 62 and 63 of a map can never be visible.

Sprites are not affected by scrolling. See [Sprites](./sprites.md#position).

## Layer Registers

Each layer has 8 bytes of registers:

| Offset | Size | Field |
|---|---|---|
| `+0` | 1 byte | Scroll X |
| `+1` | 1 byte | Scroll Y |
| `+2` | 1 byte | Blend mode (see [Blending](./blending.md)) |
| `+3` | 1 byte | [Flags](#flags) |
| `+4` | 4 bytes | *Reserved* |

The address of each layer's registers is in [VRAM Layout](./vram-layout.md).

### Flags

*Offset `+3`*

| Bit | Flag |
|---|---|
| `0` | Enabled: `1` means the layer is drawn, `0` means the layer is skipped |
| `1` to `7` | *Reserved* |

A disabled layer is skipped entirely, but its scroll and blend mode values are kept. Sprites are still drawn in the same order relative to where the layer would have been (see the [Overview](./overview.md#draw-order)).
