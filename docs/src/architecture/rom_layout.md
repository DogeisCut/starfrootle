# ROM Layout


| Offset | Size | Vector |
|---|---|---|
| `+0x00` | 4 bytes | Magic Number (`53 54 52 4C` STRL) |
| `+0x04` | 1 byte | Fomat version (`0x00` Currently) |
| `+0x05` | 11 byte | *Reserved* |
| `+0x10` | 4 bytes | Reset vector |
| `+0x14` | 4 bytes | VBlank vector |
| `+0x18` | 4 bytes | Scanline vector |
| `+0x1C` | 36 bytes | *Reserved for future vectors* |
| `+0x40` | rest | Code and data |

All vectors have 1 byte of space after them.