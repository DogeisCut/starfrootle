# Blending

> [!NOTE]
> This page is currently a stub and needs more detail.

> [!WARNING]
> Blending is purposely left undefined for now. The math behind each mode, the order blending is applied in, and how modes like Clip behave are all still being designed, and will be documented here once they are decided.

## Available Modes

| Value | Mode | Sprites | Tile layers |
|---|---|---|---|
| `0` | Normal | Yes | Yes |
| `1` | Add | Yes | Yes |
| `2` | Multiply | Yes | Yes |
| `3` | Invert | Yes | Yes |
| `4` | Clip | Yes | No |

Currently only these 5 modes exist. Future modes may be added. Values above the listed modes are not yet defined.

## Where the Blend Mode is Stored

- **Sprites** store a blend mode in the low nybble of offset `+4` of their [sprite entry](./sprites.md#blend-mode).
- **Tile layers** store a blend mode in offset `+2` of their [layer registers](./tile-layers.md#layer-registers). Individual tiles don't have a blend mode.
