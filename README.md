# kettle

A CHIP-8 emulator written in Rust with an SDL2 frontend.

## Features

- Full CHIP-8 instruction set
- 64x32 display, scaled 15x
- Keyboard mapped to the 16-key hex keypad
- 60Hz delay and sound timers
- Emulator core separated from the frontend

## Structure

```
kettle/
├── Cargo.toml        workspace root
├── kettle_core/      emulator core (CPU, memory, display buffer, timers)
└── frontend/         SDL2 window, input, and rendering
```

`kettle_core` has no window or graphics dependencies, so it can be reused with any renderer.

## Requirements

- Rust (stable)
- SDL2, installed on your system or built from source with the `bundled` feature (needs `cmake`)

On macOS:

```
brew install sdl2
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

To build SDL2 from source instead, set this in `frontend/Cargo.toml`:

```toml
sdl2 = { version = "0.37", features = ["bundled"] }
```

If CMake 4.x fails with a policy error, prefix the build with `CMAKE_POLICY_VERSION_MINIMUM=3.5`.

## Run

```
cargo run --release -- path/to/game.ch8
```

The `--` separates cargo's arguments from the ROM path.

## Controls

```
CHIP-8        Keyboard
1 2 3 C       1 2 3 4
4 5 6 D       Q W E R
7 8 9 E       A S D F
A 0 B F       Z X C V
```

Press `Esc` to quit.

## Speed

The frontend runs 10 instructions per frame at vsync, about 600 instructions per second on a 60Hz display. Change `TICKS_PER_FRAME` in `frontend/src/main.rs` to speed games up or slow them down.

## Implementation notes

- 4096 bytes of RAM, programs loaded at `0x200`, font at `0x000`
- `8XY6` and `8XYE` shift `VX` in place and ignore `VY`
- `FX55` and `FX65` leave `I` unchanged
- `DXYN` wraps each pixel around the screen edges

## Known limitations

- Sound is not implemented. The sound timer counts down but nothing plays.
- Unimplemented opcodes panic.

## Resources
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Timendus's CHIP-8 test suite](https://github.com/Timendus/chip8-test-suite)
