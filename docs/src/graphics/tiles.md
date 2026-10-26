# Tiles

> [!NOTE]
> This page is currently a stub and needs more detail.

Tiles are 8x8 [images](./images.md). Tiles are stored in the tile maps of [tile layers](./tile-layers.md), and a tile map entry is 4 bytes:

| Offset | Size    | Field                                         |
|--------|---------|-----------------------------------------------|
| `+0`   | 3 bytes | Image address                                 |
| `+3`   | 1 byte  | Palette (high nybble), Transform (low nybble) |

This is the same format as the first 4 bytes of a [sprite entry](./sprites.md#layout).

**Image address** (`+0` to `+2`): the address of the image this tile reads as image data.

**Palette** (`+3`, high nybble): which of the 16 [palettes](./palettes.md) this tile is using.

**Transform** (`+3`, low nybble): a transformation applied to the tile's image when it is drawn. The values are listed under [Images](./images.md#transformations).

Tiles have no blend mode of their own. Blending is set for a whole layer instead, see [Tile Layers](./tile-layers.md#layer-registers).
