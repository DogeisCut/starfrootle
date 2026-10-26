# Instruction Set

> [!NOTE]
> This page is currently a stub and needs more detail. Operands, flag effects, byte lengths and cycle counts are not documented yet.

> [!WARNING]
> This is not the final instruction set. While opcodes are very unlikely to change, new opcodes/instructions will be created as Starfrootle develops. 

Opcodes are a single byte, so there can be at most 255 different instructions. Opcodes are grouped by their high nybble. Any byte that is not a defined opcode behaves like `NoOp`.

| High nybble | Group                        |
|-------------|------------------------------|
| `0x`        | System                       |
| `1x`        | Loads and stores             |
| `2x`        | Moves, pushes and pops       |
| `3x`        | Arithmetic                   |
| `4x`        | Logic and shifts             |
| `5x`        | Bit operations *(planned)*   |
| `6x`        | Comparisons                  |
| `7x`        | Jumps and branches           |
| `8x`        | Calls and returns            |
| `9x`        | Block operations *(planned)* |

## Defined opcodes

| Opcode | Name             | Operands       | Bytes | Flags   | Effect                                       |
|--------|------------------|----------------|-------|---------|----------------------------------------------|
| `00`   | `NoOp`           | none           | 1     | none    | do nothing                                   |
| `01`   | `Halt`           | none           | 1     | none    | stop the CPU                                 |
| `10`   | `LoadConst`      | `D_` `value`   | 3     | none    | register = value                             |
| `11`   | `LoadMem`        | `D_` `address` | 5     | none    | register = byte at address                   |
| `12`   | `StoreMem`       | `S_` `address` | 5     | none    | byte at address = register                   |
| `20`   | `Copy`           | `DS`           | 2     | none    | D = S                                        |
| `21`   | `Swap`           | `DS`           | 2     | none    | exchange D and S                             |
| `22`   | `Push`           | `S_`           | 2     | none    | push register onto the stack                 |
| `23`   | `Pop`            | `D_`           | 2     | none    | pop the stack into register                  |
| `30`   | `Add`            | `DS`           | 2     | Z C N V | D = D + S                                    |
| `31`   | `Sub`            | `DS`           | 2     | Z C N V | D = D - S                                    |
| `32`   | `Inc`            | `D_`           | 2     | Z N V   | D = D + 1                                    |
| `33`   | `Dec`            | `D_`           | 2     | Z N V   | D = D - 1                                    |
| `40`   | `And`            | `DS`           | 2     | Z N     | D = D & S                                    |
| `41`   | `Or`             | `DS`           | 2     | Z N     | D = D \| S                                   |
| `42`   | `Xor`            | `DS`           | 2     | Z N     | D = D ^ S                                    |
| `43`   | `Not`            | `D_`           | 2     | Z N     | D = ~D                                       |
| `44`   | `ShiftLeft`      | `Dn`           | 2     | Z C N   | D <<= n                                      |
| `45`   | `ShiftRight`     | `Dn`           | 2     | Z C N   | D >>= n                                      |
| `60`   | `Compare`        | `DS`           | 2     | Z C N V | computes D - S, only sets flags              |
| `70`   | `Jump`           | `address`      | 4     | none    | PC = address                                 |
| `71`   | `JumpIfZero`     | `address`      | 4     | none    | jump if Z set                                |
| `72`   | `JumpIfNotZero`  | `address`      | 4     | none    | jump if Z clear                              |
| `73`   | `JumpIfCarry`    | `address`      | 4     | none    | jump if C set                                |
| `74`   | `JumpIfNoCarry`  | `address`      | 4     | none    | jump if C clear                              |
| `75`   | `JumpIfNegative` | `address`      | 4     | none    | jump if N set                                |
| `80`   | `Call`           | `address`      | 4     | none    | push return address (3 bytes), jump          |
| `81`   | `Return`         | none           | 1     | none    | pop PC                                       |
