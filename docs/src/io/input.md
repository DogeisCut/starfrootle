# Input

> [!NOTE]
> This page is currently a stub and needs more detail.

## Controller Entry

A controller entry is exactly 16 bytes. Controller N lives at `base + N * 16`, where `base` is the start of the
controller table (see [I/O Layout](./io_layout.md)).

A controller entry in memory can be modified but will be overwritten by the hardware each frame.

### Layout

| Offset            | Size    | Field                                                                      |
|-------------------|---------|----------------------------------------------------------------------------|
| `+0` low nybble   |         | Face buttons: South, East, West, North (bits 0 to 3)                       |
| `+0` high nybble  | 1 byte  | Start, Select, Left stick click, Right stick click                         |
| `+1` low nybble   |         | D-pad: Up, Down, Left, Right                                               |
| `+1` high nybble  | 1 byte  | Left bumper, right bumper, left trigger (digital), right trigger (digital) |
| `+2`              | 1 byte  | Left stick X                                                               |
| `+3`              | 1 byte  | Left stick Y                                                               |
| `+4`              | 1 byte  | Right stick X                                                              |
| `+5`              | 1 byte  | Right stick Y                                                              |
| `+6`              | 1 byte  | Left trigger                                                               |
| `+7`              | 1 byte  | Right trigger                                                              |
| `+8`              | 1 byte  | Battery                                                                    |
| `+9`              | 1 byte  | Flags                                                                      |
| `+10`             | 6 bytes | *Reserved*                                                                 |

### Face Buttons

### Other Buttons

### D-pad

### Bumpers and Triggers

### Triggers (analogue)

### Sticks

### Battery

A percentage from 0 to 255 that tells you how much battery this controller has left. A controller with no battery level
will always return 255.

### Flags

| Bit         | Flag                    |
|-------------|-------------------------|
| `0`         | Controller connected    |
| `1`         | Controller uses battery |
| `2` to `7`  | Reserved                |