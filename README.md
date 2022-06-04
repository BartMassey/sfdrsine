# sfdrsine: Find a "good" integer sine wave
Bart Massey 2022

A recent [Reddit post](https://www.reddit.com/r/DSP/comments/v3rzfl/a_vaguely_interesting_14bit_samples/) asked for a 14-bit signed sine wave
with frequency f/16 and low
[SFDR](https://en.wikipedia.org/wiki/Spurious-free_dynamic_range).

The Rust code here finds the same solution found in the
Reddit post:

    0, 3107, 5741, 7501, 8119, 7501, 5741, 3107,
    0, -3107, -5741, -7501, -8119, -7501, -5741, -3107,

with -103.39690314160477 dB SFDR relative to a sine wave
with amplitude 8119.0078125 and 0 phase. It runs in about
4.3s on my 12-core box.
