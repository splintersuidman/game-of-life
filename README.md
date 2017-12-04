# Game of Life [![Build Status](https://travis-ci.org/splintah/game-of-life.svg?branch=master)](https://travis-ci.org/splintah/game-of-life)
A Game of Life emulation, both in a graphical window and in your terminal.

![Window Demo](./window-demo.png)

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
```
Press Escape to exit, press Space or a mouse button to reinitialise grid randomly.

Flags:
  --help | -h
    Show this screen.
  --width [w] : u32
    Change the width of the board (in cells).
    Default: 50.
  --height [h] : u32
    Change the height of the board (in cells).
    Default: 50.
  --cell-width [cw] : u32
    Change width of a cell (in pixels).
    Default: 10.
  --chance [c] : u8
    Chance for randomly initialising board.
    Example: with '--chance 128' passed, cells will have a 50% chance of living.
    Default: 220.
  --chance [c] : Option<u64>
    The amount of milliseconds the thread sleeps between every frame.
    Default: None.
```

### CLI
```
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
```
