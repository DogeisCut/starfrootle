# Sprites

> [!NOTE]
> This page is currently a stub and needs more detail.

## Sprite Entry

A sprite entry is exactly 8 bytes. Sprite N lives at `base + N * 8`, where `base` is the start of the sprite table (see [VRAM Layout](./vram-layout.md)).

A single sprite may look like this in memory:

`C0 00 00 47 40 FF FF 00`

### Layout

| Offset | Size | Field |
|---|---|---|
| `+0` | 3 bytes | [Image address](#image-address) |
| `+3` | 1 byte | [Palette](#palette-and-transform) (high nybble), [Transform](#palette-and-transform) (low nybble) |
| `+4` | 1 byte | [Layer offset](#layer-offset) (high nybble), [Blend mode](#blend-mode) (low nybble) |
| `+5` | 2 bytes | [X and Y position](#position) |
| `+7` | 1 byte | *Reserved* |

### Image Address

*Offsets `+0` to `+2`* (`C0 00 00`)

An address that defines what section of memory this sprite will read as image data.

This image data is read as an 8x8 indexed image. See [Images](./images.md) for more information.

### Palette and Transform

*Offset `+3`* (`47`)

| High nybble | Low nybble |
|---|---|
| Palette | Transform |

**Palette** (`4`): which of the 16 palettes this sprite is using.

**Transform** (`7`): a transformation applied to the sprite.

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

### Layer Offset

*Offset `+4`, high nybble* (`4`)

The layer offset places the sprite relative to the tile layers. Layer 0 is the topmost tile layer.

| Value | Sprite is drawn... |
|---|---|
| `0` | In front of tile layer 0 |
| `1` | Behind tile layer 0 |
| `2` | In front of tile layer 1, behind anything on layer 0 |
| `3` | Behind tile layer 1 |
| `4` | In front of tile layer 2, behind anything on layer 1 |
| `5` | Behind tile layer 2 |
| `6` | In front of tile layer 3, behind anything on layer 2 |
| `7` | Behind tile layer 3 |
| `8` | In front of tile layer 4, behind anything on layer 3 |
| `9` | Behind tile layer 4 |

Values `A` to `F` are not yet defined.

### Blend Mode

*Offset `+4`, low nybble* (`0`)

The blend mode affects how the sprite blends with the pixels behind it.

> [!WARNING]
> The exact math behind sprite blending modes is not defined here yet. See [Blending](./blending.md).

| Value | Mode |
|---|---|
| `0` | Normal |
| `1` | Add |
| `2` | Multiply |
| `3` | Invert |
| `4` | Clip (this sprite's pixels will only show up on top of other sprites' pixels, regardless of the layer) |

Currently only these 5 modes are implemented. Future modes may be added. Values above the defined range default to the same behavior as Normal.

### Position

*Offsets `+5` to `+6`* (`FF FF`)

The game screen has a 8 pixel margin around it, sprites can hide beind these.
Sprite positions are screen-spaced, meaning any tilemap scrolling will not scroll sprites with it.

- **X** (`FF`, from `+5`): sprites start 8 pixels outside the left of the screen. This value shifts the sprite right along the X axis, in pixels.
- **Y** (`FF`, from `+6`): sprites start 8 pixels outside the top of the screen. This value shifts the sprite down along the Y axis, in pixels.

If you wish to hide a sprite or aren't using it, parking it in any corner outside the screen is the correct way to do it. Most commonly, this is done at the top left corner, `0,0`.

## Example

`C0 00 00 47 40 FF FF 00` decodes to:

| Field | Value |
|---|---|
| Image address | `0xC00000` |
| Palette | `4` |
| Transform | `7` (horizontally flipped, then 270 degrees clockwise) |
| Layer offset | `4` (in front of tile layer 2, behind anything on layer 1) |
| Blend mode | `0` (Normal) |
| X | `0xFF` (255) |
| Y | `0xFF` (255) |
| *Reserved* | `0x00` (0) |
