# Audio

> [!NOTE]
> This page is currently a stub and needs more detail.

Starfrootle has 16 audio channels. Every channel is a mono source with a pan control, and all channels are mixed into a stereo output at 48,000 Hz (800 samples per frame at 60 FPS). Pan the center to get a mono sound, or play a stereo sound on two channels panned hard left and hard right.

The CPU only writes to registers. The console generates the sound itself, and music and sound effects are handled by code in the game.

Where the audio registers live in memory is described in [I/O Layout](./io-layout.md#audio).

## Signal Flow

Each channel passes through the following steps, in order:

1. **Source**: a generated wave, noise, or a sample.
2. **Filter**: a low-pass filter (only if the filter is enabled).
3. **Envelope and volume**: the envelope shapes the volume over time (only if the envelope is enabled).
4. **Effect sends**: a part of the signal is sent to the global reverb, chorus and delay.
5. **Pan**: the signal is placed between the left and right outputs.

The three global effects are mixed with the channels to make the final output, which is then scaled by the master volume.

## Channel Entry

A channel entry is exactly 64 bytes. Channel N lives at `base + N * 0x40`, where `base` is the start of the audio channel table (see [I/O Layout](./io-layout.md#audio)).

All of the multi-byte values in a channel entry are big-endian. Zeroed memory is silent.

### Layout

| Offset  | Size     | Field                            |
|---------|----------|----------------------------------|
| `+0x00` | 1 byte   | [Flags](#flags)                  |
| `+0x01` | 1 byte   | [Source](#source-and-variant)    |
| `+0x02` | 1 byte   | [Variant](#source-and-variant)   |
| `+0x03` | 1 byte   | [Volume](#volume-and-pan)        |
| `+0x04` | 1 byte   | [Pan](#volume-and-pan) (signed)  |
| `+0x05` | 2 bytes  | [Pitch](#pitch)                  |
| `+0x07` | 1 byte   | [Envelope](#envelope): attack    |
| `+0x08` | 1 byte   | Envelope: decay                  |
| `+0x09` | 1 byte   | Envelope: sustain level          |
| `+0x0A` | 1 byte   | Envelope: release                |
| `+0x0B` | 1 byte   | [Vibrato](#vibrato): rate        |
| `+0x0C` | 1 byte   | Vibrato: depth                   |
| `+0x0D` | 1 byte   | Vibrato: shape                   |
| `+0x0E` | 1 byte   | [Filter](#filter): cutoff        |
| `+0x0F` | 1 byte   | Filter: peak                     |
| `+0x10` | 1 byte   | [Reverb send](#effect-sends)     |
| `+0x11` | 1 byte   | Chorus send                      |
| `+0x12` | 1 byte   | Delay send                       |
| `+0x13` | 1 byte   | *Reserved*                       |
| `+0x14` | 3 bytes  | [Sample](#samples) start address |
| `+0x17` | 3 bytes  | Sample end address               |
| `+0x1A` | 3 bytes  | Sample loop start address        |
| `+0x1D` | 3 bytes  | Sample loop end address          |
| `+0x20` | 32 bytes | *Reserved*                       |

### Flags

*Offset `+0x00`*

| Bit        | Flag                                                                                                                           |
|------------|--------------------------------------------------------------------------------------------------------------------------------|
| `0`        | Gate: a note starts when this changes from `0` to `1`, and is released when it changes from `1` to `0`                         |
| `1`        | Loop (samples only)                                                                                                            |
| `2`        | Envelope enabled                                                                                                               |
| `3`        | Filter enabled                                                                                                                 |
| `4`        | Active (read only): `1` while the channel is making sound. The console sets and clears this bit, and writes to it are ignored. |
| `5` to `7` | *Reserved*                                                                                                                     |

When a note starts, the envelope begins its attack from silence, the vibrato starts at the beginning of its cycle, the filter is cleared, and a sample starts playing from its start address.

When a note is released, the envelope starts its release. If the envelope is not enabled, the sound stops immediately. The active bit is cleared once the sound has fully stopped, which includes a sample reaching its end when it isn't looping. To play a note again after that, write `0` and then `1` to the gate.

### Source and Variant

*Offsets `+0x01` and `+0x02`*

| Source | Sound    |
|--------|----------|
| `0`    | Pulse    |
| `1`    | Triangle |
| `2`    | Saw      |
| `3`    | Sine     |
| `4`    | Noise    |
| `5`    | Sample   |

Values above `5` are not yet defined.

The variant changes how some sources sound, and is ignored by the others:

| Source | Variant                                                                                                        |
|--------|----------------------------------------------------------------------------------------------------------------|
| Pulse  | Duty cycle: the fraction of each wave that is high, as the value divided by 256 (`0x80` is 50%, `0x40` is 25%) |
| Noise  | `0` is long noise, `1` is short, more metallic noise. Other values are not yet defined.                        |

### Volume and Pan

*Offsets `+0x03` and `+0x04`*

**Volume** goes from `0` (silent) to `255` (full volume).

**Pan** is a signed byte from `-128` (fully left) through `0` (center) to `127` (fully right). At the center, both sides play at full volume. Moving away from the center fades the opposite side out linearly.

### Pitch

*Offsets `+0x05` and `+0x06`*

Pitch is a 16-bit value that sets the playback speed as a multiple of the sample's native speed:

`speed = pitch / 0x1000`

So `0x1000` plays at normal speed, `0x2000` plays one octave higher, and `0x0800` plays one octave lower. A pitch of `0x0000` is silent. The values are linear in frequency, so going up an octave doubles the value, which is a shift left.

Samples are expected to be tuned to C-5 and have a native rate of 32,000 Hz, so `0x1000` is C-5. Generated waves at `0x1000` also produce C-5 (about 523.25 Hz), and scale linearly from there. Noise advances at the same ratio.

Notes follow equal temperament with A-4 at 440 Hz. This is the table for octave 5, and every other octave is this table shifted left (up) or right (down) once per octave:

| Note | Pitch    | Note | Pitch    |
|------|----------|------|----------|
| C-5  | `0x1000` | F#-5 | `0x16A1` |
| C#-5 | `0x10F4` | G-5  | `0x17F9` |
| D-5  | `0x11F6` | G#-5 | `0x1966` |
| D#-5 | `0x1307` | A-5  | `0x1AE9` |
| E-5  | `0x1429` | A#-5 | `0x1C82` |
| F-5  | `0x155C` | B-5  | `0x1E34` |

The range of notes that fit is C-0 (`0x0080`) to B-8 (`0xF1A2`). One step of pitch is about 0.42 cents at C-5, and is coarser for lower notes.

### Envelope

*Offsets `+0x07` to `+0x0A`*

The envelope shapes the volume of a note, and only applies if the envelope flag is set. Without it, a note plays at full volume until it is released.

| Offset  | Field         | Meaning                                                                     |
|---------|---------------|-----------------------------------------------------------------------------|
| `+0x07` | Attack        | Frames to rise from silence to full volume                                  |
| `+0x08` | Decay         | Frames to fall from full volume to the sustain level                        |
| `+0x09` | Sustain level | The level held while the gate is `1`, from `0` (silent) to `255` (full)     |
| `+0x0A` | Release       | Frames to fall from the current level to silence after the gate is released |

A frame is 1/60 of a second, and a time of `0` is instant. All of the changes are linear.

### Vibrato

*Offsets `+0x0B` to `+0x0D`*

Vibrato is a low frequency oscillator (LFO) that moves the pitch up and down.

| Offset  | Field | Meaning                                                                                                                               |
|---------|-------|---------------------------------------------------------------------------------------------------------------------------------------|
| `+0x0B` | Rate  | Cycles per second, as the value divided by 16 (`0x60` is 6 Hz)                                                                        |
| `+0x0C` | Depth | The peak change in pitch in semitones, as the value divided by 16 (`0x10` is one semitone up and down). A depth of `0` is no vibrato. |
| `+0x0D` | Shape | `0` sine, `1` triangle, `2` saw, `3` square. Other values are not yet defined.                                                        |

The depth is measured in semitones, so vibrato sounds the same in every octave.

### Filter

*Offsets `+0x0E` and `+0x0F`*

A two pole low-pass filter that only applies if the filter flag is set.

| Offset  | Field  | Meaning                                                                                                                       |
|---------|--------|-------------------------------------------------------------------------------------------------------------------------------|
| `+0x0E` | Cutoff | Cutoff frequency, from 20 Hz at `0` rising exponentially to about 20 kHz at `255`. The frequency is 20 Hz × 2^(value / 25.5). |
| `+0x0F` | Peak   | Resonance peak at the cutoff, from `0` (no peak) to `255` (the strongest peak), roughly a Q of 0.7 to 10                      |

### Effect Sends

*Offsets `+0x10` to `+0x12`*

Each send is how much of this channel's sound goes to the matching [global effect](#global-block), from `0` (none) to `255` (full). The send is taken after the envelope and volume are applied, and before pan.

### Samples

*Offsets `+0x14` to `+0x1F`*

A sample is a run of 8-bit signed mono samples, one byte per sample, with a native rate of 32,000 Hz. Samples are read straight from memory, usually ROM. Playback uses linear interpolation.

| Offset  | Field                                                                                                 |
|---------|-------------------------------------------------------------------------------------------------------|
| `+0x14` | Start address: where playback begins when a note starts                                               |
| `+0x17` | End address: the address just after the last sample. Playback stops here unless the loop flag is set. |
| `+0x1A` | Loop start address                                                                                    |
| `+0x1D` | Loop end address: the address just after the last looped sample                                       |

When the loop flag is set, playback jumps back to the loop start whenever it reaches the loop end, and the end address is not used. Samples can have an intro before the loop start.

## Global Block

The global block holds settings for the whole mix. It lives right after the channel table (see [I/O Layout](./io-layout.md#audio)).

| Offset  | Size    | Field          | Meaning                                                                                 |
|---------|---------|----------------|-----------------------------------------------------------------------------------------|
| `+0x00` | 1 byte  | Master volume  | `0` (silent) to `255` (full)                                                            |
| `+0x01` | 1 byte  | Reverb return  | How loud the reverb is in the mix, `0` to `255`                                         |
| `+0x02` | 1 byte  | Reverb decay   | How long the reverb lasts, `0` (shortest) to `255` (longest)                            |
| `+0x03` | 1 byte  | Chorus return  | How loud the chorus is in the mix, `0` to `255`                                         |
| `+0x04` | 1 byte  | Chorus rate    | Speed of the chorus, in the same units as the vibrato rate (value divided by 16, in Hz) |
| `+0x05` | 1 byte  | Chorus depth   | Amount of the chorus, `0` (none) to `255` (most)                                        |
| `+0x06` | 1 byte  | Delay return   | How loud the delay is in the mix, `0` to `255`                                          |
| `+0x07` | 1 byte  | Delay time     | Time between echoes, in units of 5 milliseconds (up to 1.275 seconds)                   |
| `+0x08` | 1 byte  | Delay feedback | How much of each echo is fed back into the delay, as the value divided by 256           |
| `+0x09` | 7 bytes | *Reserved*     |                                                                                         |

Since zeroed memory has a master volume of `0`, a game has to set the master volume before it will hear anything.

> [!WARNING]
> The exact algorithms behind the reverb, chorus and delay are purposely left undefined for now. Only their settings are defined here.
