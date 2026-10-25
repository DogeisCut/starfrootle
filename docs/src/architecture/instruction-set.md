# Instruction Set

> [!NOTE]
> This page is currently a stub and needs more detail. Operands, flag effects, byte lengths and cycle counts are not documented yet.

Opcodes are a single byte, so there can be at most 255 different instructions. Opcodes are grouped by their high nybble. Any byte that is not a defined opcode behaves like `NoOp`.

| High nybble | Group |
|---|---|
| `0x` | System |
| `1x` | Loads and stores |
| `2x` | Moves, pushes and pops |
| `3x` | Arithmetic |
| `4x` | Logic and shifts |
| `5x` | Bit operations *(planned)* |
| `6x` | Comparisons |
| `7x` | Jumps and branches |
| `8x` | Calls and returns |
| `9x` | Block operations *(planned)* |

## Defined opcodes

| Opcode | Name |
|---|---|
| `00` | `NoOp` |
| `01` | `Halt` |
| `10` | `LoadConst` |
| `11` | `LoadMem` |
| `12` | `StoreMem` |
| `20` | `Copy` |
| `21` | `Swap` |
| `22` | `Push` |
| `23` | `Pop` |
| `30` | `Add` |
| `31` | `Sub` |
| `32` | `Inc` |
| `33` | `Dec` |
| `40` | `And` |
| `41` | `Or` |
| `42` | `Xor` |
| `43` | `Not` |
| `44` | `ShiftLeft` |
| `45` | `ShiftRight` |
| `60` | `Compare` |
| `70` | `Jump` |
| `71` | `JumpIfZero` |
| `72` | `JumpIfNotZero` |
| `73` | `JumpIfCarry` |
| `74` | `JumpIfNoCarry` |
| `75` | `JumpIfNegative` |
| `80` | `Call` |
| `81` | `Return` |
