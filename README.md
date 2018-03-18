# Game of Life [![Build Status](https://travis-ci.org/splintah/game-of-life.svg?branch=master)](https://travis-ci.org/splintah/game-of-life)

A Game of Life emulation, both in a graphical window and in your terminal, with support for several file formats.

![Window Demo](./window-demo.png)

## File support

This program supports the following file formats:

- [Life 1.06 file](http://www.conwaylife.com/wiki/Life_1.06)
- [Life 1.05 file](http://www.conwaylife.com/wiki/Life_1.05)
- [Run Length Encoded file](http://www.conwaylife.com/wiki/Run_Length_Encoded)
- [Plaintext file](http://www.conwaylife.com/wiki/Plaintext)

You can find a lot of patterns, and their files, on the [LifeWiki](http://www.conwaylife.com/wiki).
There is a list of patterns [here](http://www.conwaylife.com/wiki/Category:Patterns).
To use a file, pass it to the `--file` flag (see [below](#usage) for more flags).

## Installation

[Install Rust](https://www.rust-lang.org/en-US/). Then run the following in your terminal:

```bash
git clone https://github.com/splintah/game-of-life.git
cd game-of-life
cargo build --release
```

The executables are now located in `target/release/`.
You may want to move these executables into a folder in your path.

## Usage

### Window

```text
Press Escape to exit, press Space or a mouse button to reinitialise grid.

Flags:
  --help
    Show this screen.
  --width [w] | -w [w]
    : u32
    Change the width of the board (in cells).
    Default: 50.
  --height [h] | -h [h]
    : u32
    Change the height of the board (in cells).
    Default: 50.
  --cell-width [cw] | -cw [cw]
    : u32
    Change width of a cell (in pixels).
    Default: 10.
  --chance [c] | -ch [ch]
    : u8
    Chance for randomly initialising board.
    Example: with '--chance 128' passed, cells will have a 50% chance of living.
    Default: 220.
  --fps [f] | -fps [f]
    : u64
    The amount of updates and frames that should be performed per second.
    This is the maximum frames per second; that is, the actual fps could be less.
    Default: 24.
  --file [f] | -f [f]
    : path
    The file that contains the board.
    If this flag is passed, the board will be initialised with the board in the given file.
    Default: None.
  --colour [c] | --color [c] | -c [c]
    : u32
    Change the foreground colour of the cells.
    The colour should be passed as a hexidecimal RGB colour, example: FFFFFF for white, 000000 for black.
    Default: FFFFFF (white).
  --background [c] | -bg [c]
    : u32
    Change the background colour.
    The colour should be passed as a hexidecimal RGB colour, example: FFFFFF for white, 000000 for black.
    Default: 000000 (black).
```

### CLI

```text
Flags:
  --help | -h
    Show this screen.
  --width [w] : u16
    Change the width of the board.
    Default: terminal width.
  --height [h] : u16
    Change the height of the board.
    Default: terminal height.
  --chance [c] : u8
    Chance for randomly initialising board.
    Example: with '--chance 128' passed, cells will have a 50% chance of living.
    Default: 220.
  --sleep [s] : u64
    The amount of milliseconds the thread sleeps between every frame.
    Default: None.
  --file [f] : path
    The file that contains the board.
    If this flag is passed, the board will be initialised with the board in the given file.
    Default: None.
```
