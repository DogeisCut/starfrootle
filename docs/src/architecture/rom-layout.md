# ROM Layout

> [!NOTE]
> This page is currently a stub and needs more detail.

| Offset  | Size     | Field                             |
|---------|----------|-----------------------------------|
| `+0x00` | 4 bytes  | Magic Number (`53 54 52 4C` STRL) |
| `+0x04` | 1 byte   | Format version (`0x00` Currently) |
| `+0x05` | 8 bytes  | Unique Game ID                    |
| `+0x0D` | 3 bytes  | *Reserved*                        |
| `+0x10` | 4 bytes  | Reset vector                      |
| `+0x14` | 4 bytes  | VBlank vector                     |
| `+0x18` | 4 bytes  | Scanline vector                   |
| `+0x1C` | 36 bytes | *Reserved for future vectors*     |
| `+0x40` | rest     | Code and data                     |

Each vector is a 3-byte address followed by 1 reserved byte.