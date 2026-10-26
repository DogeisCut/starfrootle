# VRAM Layout

> [!NOTE]
> This page is currently a stub and needs more detail.

VRAM starts at `0x400000`. Each region has a slot that is larger than what it currently uses, so a region can grow without the others having to move.

| Offset | Size | Used | Contents |
|--------|------|------|----------|
| `+0x00000` | `0x1000` | `0x400` | [Sprite table](#sprite-table) |
| `+0x01000` | `0x1000` | `0x30` | [Background color](#background-color) and [layer registers](#layer-registers) |
| `+0x02000` | `0x2000` | `0x300` | [Palettes](#palettes) |
| `+0x04000` | `0x4000` | `0x4000` | [Tile map](#tile-maps) for layer 0 |
| `+0x08000` | `0x4000` | `0x4000` | Tile map for layer 1 |
| `+0x0C000` | `0x4000` | `0x4000` | Tile map for layer 2 |
| `+0x10000` | `0x4000` | `0x4000` | Tile map for layer 3 |
| `+0x14000` | `0x4000` | `0x4000` | Tile map for layer 4 |
| `+0x18000` | `0x8000` | `0` | *Reserved* |

## Sprite Table

128 [sprite entries](./sprites.md) of 8 bytes each, which is 1 KB. Sprite N lives at `vram start + N * 8`.

## Background Color

The background color is 3 bytes, red then green then blue, at `+0x01000`. It is drawn behind everything and is not part of any [palette](./palettes.md). The 5 bytes after it (`+0x01003` to `+0x01007`) are reserved.

## Layer Registers

Each layer has 8 bytes of [layer registers](./tile-layers.md#layer-registers). The registers for layer N live at `vram start + 0x01008 + N * 8`.

## Palettes

16 [palettes](./palettes.md) of 16 colors, 3 bytes per color. Color C of palette P is at `vram start + 0x02000 + P * 0x30 + C * 3`.

## Tile Maps

Each [tile layer](./tile-layers.md) has a tile map of 64x64 [tile entries](./tiles.md), 4 bytes each. The tile map for layer N starts at `vram start + 0x04000 + N * 0x4000`, and the entry at column X and row Y is at `map start + Y * 0x100 + X * 4`.
