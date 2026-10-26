# Overview

> [!NOTE]
> This page is currently a stub and needs more detail.

- 240x240 screen size (30 tiles across, 30 tiles vertically)
- 8x8 tiles and sprites
- 5 [tile layers](./tile-layers.md) (layer 0 is the topmost), each with a 64x64 tile map and its own scrolling
- 128 [sprites](./sprites.md)
- 16 [palettes](./palettes.md)
- A background color, which is separate from the palettes

Where each of these lives in memory is described in [VRAM Layout](./vram-layout.md).

## Draw Order

Everything on screen is drawn back to front:

| Step | Drawn                                   |
|------|-----------------------------------------|
| 1    | Background color                        |
| 2    | Sprites with layer offset `9`           |
| 3    | Tile layer 4                            |
| 4    | Sprites with layer offset `8`, then `7` |
| 5    | Tile layer 3                            |
| 6    | Sprites with layer offset `6`, then `5` |
| 7    | Tile layer 2                            |
| 8    | Sprites with layer offset `4`, then `3` |
| 9    | Tile layer 1                            |
| 10   | Sprites with layer offset `2`, then `1` |
| 11   | Tile layer 0                            |
| 12   | Sprites with layer offset `0`           |

Sprites that share a layer offset are drawn in the order they appear in the sprite table, so later sprites are drawn on top.

A tile layer that is disabled (see [Layer Registers](./tile-layers.md#flags)) is skipped, and everything else is drawn in the same order.

How things blend as they are drawn is purposely undefined for now. See [Blending](./blending.md).
