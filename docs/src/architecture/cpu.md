# CPU

> [!NOTE]
> This page is currently a stub and needs more detail.

## Registers

| Register             | Size        | Notes                                                 |
|----------------------|-------------|-------------------------------------------------------|
| General purpose (16) | 8 bits each | Numbered `0` to `F`, so a register fits in one nybble |
| Program counter      | 24 bits     | Address of the next byte to read                      |
| Stack pointer        | 24 bits     |                                                       |
| Flags                | 8 bits      | See below                                             |

## Flags

| Bit | Flag     |
|-----|----------|
| `0` | Zero     |
| `1` | Carry    |
| `2` | Negative |
| `3` | Overflow |

Bits `4` to `7` are currently unused.
