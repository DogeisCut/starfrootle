# Images

> [!NOTE]
> This page is currently a stub and needs more detail.

An image is 8x8 pixels, made up of 32 bytes, with each nybble representing which color in the palette that specific pixel uses.

Tiles and sprites don't store pixels themselves. They store the address of an image (see [Tiles](./tiles.md) and [Sprites](./sprites.md)).

> [!NOTE]
> The order of the pixels inside an image (row order, and which nybble of a byte is the left pixel) is not documented yet.

Index `0` of a palette can be transparent. See [Palettes](./palettes.md#transparency).

## Transformations

Tiles and sprites can both apply a transformation to their image when it is drawn. A transformation is a single nybble value:

| Value | Transformation |
|---|---|
| `0` | No rotation |
| `1` | 90 degrees (clockwise) |
| `2` | 180 degrees (clockwise) |
| `3` | 270 degrees (clockwise) |
| `4` | Horizontally flipped, no rotation |
| `5` | Horizontally flipped, then 90 degrees (clockwise) |
| `6` | Horizontally flipped, then 180 degrees (clockwise) |
| `7` | Horizontally flipped, then 270 degrees (clockwise) |

Anything beyond this repeats the previous entries in the list. These may be used for something in the future.
