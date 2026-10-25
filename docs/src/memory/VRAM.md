# VRAM

> [!NOTE]
> This page is currently a stub and needs more detail.

VRAM starts at `0x400000`

- 128 sprites
- 16 palettes
- 8x8 tiles and sprites
- 5 tile layers (Layer 0 is the topmost)
- 320x240 screen size (40 tiles across, 30 tiles vertically)

## Sprites

Sprites start right at the start of VRAM.

A sprite entry is exactly 8 bytes, so sprite N lives at `base + N * 8` (with base being the start of VRAM).

The Starfrootle supports up to 128 sprites, seeing as a single sprite takes up 8 bytes, all 128 sprites take up 1 KB.

For more information, you will want to read the format for [a single sprite](./VRAM/sprites.md).