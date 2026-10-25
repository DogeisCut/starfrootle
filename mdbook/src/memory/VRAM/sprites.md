# Sprites

## Single Sprites

A single sprite may look like this in memory:

`C0 00 00 47 40 FF FF FF`

Here's what each of these bytes mean:
- **Bytes 1-3** (`C0 00 00`): An address that defines what section of memory this sprite will read as image data.
This image data is read for an 8x8 indexed sprite, an image is made up of 16 bytes, with each nybble representing
what color in the palette this specific pixel uses. 
- **Byte 4** (`47`): This byte is split up into 2 nybbles.
    - **Nybble A** (`4`): This nybble represents which of the 16 palletes this sprite is using.
    - **Nybble B** (`7`): This nybble a transformation the sprite:
        - `0`: No Rotation
        - `1`: 90 Degrees (Clockwise)
        - `2`: 180 Degrees (Clockwise)
        - `3`: 270 Degrees (Clockwise)
        - `4`: Horizontally flipped, No Rotation
        - `5`: Horizontally flipped, then 90 Degrees (Clockwise)
        - `6`: Horizontally flipped, then 180 Degrees (Clockwise)
        - `7`: Horizontally flipped, then 270 Degrees (Clockwise)
        - Anything beyond this repeats the previous entries in the list. These may be used for something in the future.
- **Byte 5** (`40`): This byte is split up into 2 nybbles.
    - **Nybble A** (`4`): This nybble represents a layer offset for this sprite.
        - `0`: In-front of tile layer 0
        - `1`: Behind of tile layer 0
        - `2`: In-front of tile layer 1
        - `3`: Behind of tile layer 1
        - `4`: In-front of tile layer 2
        - `5`: Behind of tile layer 2
        - `6`: In-front of tile layer 3
        - `7`: Behind of tile layer 3
        - `8`: In-front of tile layer 4
        - `9`: Behind of tile layer 4
    - **Nybble B** (`0`): This nybble actually represents several blending mode choices. This affects how the sprite blends with pixels behind it.
        - `0`: Normal
        - `1`: Add
        - `2`: Multiply
        - `3`: Invert
        - `4`: Clip (This sprite's pixels will only show up on top of other sprite's pixels, regardless of the layer)
        - Currently only these 5 modes are implemented. Future modes may be added. Values above these 4 modes will default to Normal blending.
- **Byte 6-8** (`FF FF FF`): These bytes are split up into 2 pairs of 3 nybbles. These represent the X and Y coords of a sprite.
    - **Nybble Triplet X** (`FF F`): Sprites start 8 pixels inside the left of the screen, this value shifts the sprite right across the X axis in pixels.
    - **Nybble Triplet Y** (`F FF`): Sprites start 8 pixels inside the top of the screen, this value shifts the sprite down across the Y axis in pixels.
