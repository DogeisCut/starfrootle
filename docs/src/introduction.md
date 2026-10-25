# Introduction

> [!NOTE]
> This page is currently a stub and needs more detail.

Starfrootle is a fantasy console written in Rust.

## Specifications

| | |
|---|---|
| Address space | 16 MB (24-bit addresses, `0x000000` to `0xFFFFFF`) |
| Screen | 240x240 (30 tiles across, 30 tiles vertically) |
| Tile and sprite size | 8x8 |
| Tile layers | 5 (each with a 64x64 tile map) |
| Sprites | 128 |
| Palettes | 16 (16 colors each) |

## Conventions

- **Big-endian.** Multi-byte values are written most significant byte first. An address of `C0 00 00` is `0xC00000`.
- **Nybbles.** A nybble is 4 bits, written as one hex digit. A byte is a pair of nybbles, and the *high* nybble is the left digit (in `47`, the high nybble is `4` and the low nybble is `7`).
- **Addresses** are 3 bytes (6 nybbles) long.
- **Hex notation.** In prose, values are written like `0x400000`. Raw bytes in memory are written as space-separated pairs, like `C0 00 00`.
