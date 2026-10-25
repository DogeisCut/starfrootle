# VRAM Layout

> [!NOTE]
> This page is currently a stub and needs more detail.

VRAM starts at `0x400000`.

| Start | Size | Contents |
|---|---|---|
| `0x400000` | `0x400` bytes (1 KB) | [Sprite table](./sprites.md): 128 entries of 8 bytes each |
| *Not yet defined* | | Palettes, tiles and tile layers |

Sprites start right at the start of VRAM. Sprite N lives at `0x400000 + N * 8`.
