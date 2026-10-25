# Sprites

## Single Sprites

A single sprite may look like this in memory:

`C0 00 00 49`

Here's what each of these bytes mean:
- **Bytes 1-3** (`C0 00 00`): An address that defines what section of memory this sprite will read as image data.
This image data is read for an 8x8 indexed sprite, an image is made up of 16 bytes, with each nybble representing
what color in the palette this specific pixel uses. 
- **Byte 4** (`49`): This byte is actually split up into 2 nybbles.
    - **Nybble A** (`4`): This nybble represents which of the 16 palletes this sprite is using.
    - **Nybble B** (`9`): This nybble a transformation the sprite:
        - `0`: No Rotation
        - `1`: 90 Degrees (Clockwise)
        - `2`: 180 Degrees (Clockwise)
        - `3`: 270 Degrees (Clockwise)
        - `4`: Horizontally flipped, No Rotation
        - `5`: Horizontally flipped, then 90 Degrees (Clockwise)
        - `6`: Horizontally flipped, then 180 Degrees (Clockwise)
        - `7`: Horizontally flipped, then 270 Degrees (Clockwise)
        - `8`: ?
        - `9`: ?
        - `A`: ?
        - `B`: ?
        - `C`: ?
        - `D`: ?
        - `E`: ?
        - `F`: ?
