# Images

> [!NOTE]
> This page is currently a stub and needs more detail.

An image is 8x8 pixels, made up of 32 bytes, with each nybble representing which color in the palette that specific pixel uses.

Tiles and sprites don't store pixels themselves. They store the address of an image (see [Tiles](./tiles.md) and [Sprites](./sprites.md)).

Index `0` of a palette can be transparent. See [Palettes](./palettes.md#transparency).

## Pixel Order

An image is stored one row at a time, starting with the top row. Each row is 4 bytes, with 2 pixels per byte. Inside a byte, the high nybble is the left pixel and the low nybble is the right pixel, the same way the byte is written.

| Offset         | Row            |
|----------------|----------------|
| `+0` to `+3`   | Row 0 (top)    |
| `+4` to `+7`   | Row 1          |
| `+8` to `+11`  | Row 2          |
| `+12` to `+15` | Row 3          |
| `+16` to `+19` | Row 4          |
| `+20` to `+23` | Row 5          |
| `+24` to `+27` | Row 6          |
| `+28` to `+31` | Row 7 (bottom) |

Within a row:

| Byte                   | `+0`      | `+1`      | `+2`      | `+3`      |
|------------------------|-----------|-----------|-----------|-----------|
| Pixels (left to right) | high, low | high, low | high, low | high, low |
| Columns                | 0, 1      | 2, 3      | 4, 5      | 6, 7      |

For example, a row of `12 34 56 78` has the palette indexes `1 2 3 4 5 6 7 8` from left to right.

## Transformations

Tiles and sprites can both apply a transformation to their image when it is drawn. A transformation is a single nybble value:

| Value | Transformation                                     |
|-------|----------------------------------------------------|
| `0`   | No rotation                                        |
| `1`   | 90 degrees (clockwise)                             |
| `2`   | 180 degrees (clockwise)                            |
| `3`   | 270 degrees (clockwise)                            |
| `4`   | Horizontally flipped, no rotation                  |
| `5`   | Horizontally flipped, then 90 degrees (clockwise)  |
| `6`   | Horizontally flipped, then 180 degrees (clockwise) |
| `7`   | Horizontally flipped, then 270 degrees (clockwise) |

Anything beyond this repeats the previous entries in the list. These may be used for something in the future.
