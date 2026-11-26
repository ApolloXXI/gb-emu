# gb-emu

A Game Boy (DMG) emulator in Rust, written to learn Rust and how the
Game Boy's Sharp LR35902 CPU works. **Early stage, currently paused.**

## Status

| Part | State |
|---|---|
| CPU registers (8-bit and 16-bit pairs, flags) | Done |
| CPU instructions | Partly: ADD, SUB, AND, OR, XOR, CP and 16-bit ADD to HL |
| Program counter / fetch-decode loop | In progress |
| Memory bus, cartridge loading | Not started |
| Graphics (PPU), timers, input | Not started |
| Front ends (`gb-cli`, `gb-sdl`) | Placeholders |

## Structure

A Cargo workspace with three crates:

```
gb-core/   emulator core: CPU, registers, instruction decoding, bus, cartridge
gb-cli/    command-line front end
gb-sdl/    SDL front end (window, input)
```

## Building

```sh
cargo build
cargo test
```
