# Palettes

> [!NOTE]
> This page is currently a stub and needs more detail.

There are 16 palettes, each with 16 colors. Palettes are shared between tiles and sprites: every pixel of an [image](./images.md) is an index (`0` to `F`) into the palette chosen by the tile or sprite that draws it.

The background color is not part of any palette. See [VRAM Layout](./vram-layout.md#background-color).

## Colors

A color is 3 bytes: red, green, then blue, with one byte per channel. A palette is 16 colors, so 48 bytes (`0x30`), and all 16 palettes take `0x300` bytes.

Color C of palette P is at `palette table + P * 0x30 + C * 3`. The address of the palette table is in [VRAM Layout](./vram-layout.md).

## Transparency

Index `0` is transparent only if its color is exactly `00 00 00` (pure black). Any other color at index `0` is drawn like every other color. No other index is ever transparent.
