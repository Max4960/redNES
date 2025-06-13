# redNES

A work-in-progress NES (Nintendo Entertainment System) emulator written in Rust. This is my implementation of the excellent [guide](https://bugzmanov.github.io/nes_ebook/chapter_1.html) by bugzmanov. All credit for the structure and explanations goes to the original author.

## Overview

`redNES` is an experimental NES emulator designed to run NES ROMs and provide a simple platform for learning about emulation and systems programming in Rust. Currently, it focuses on CPU emulation, basic memory management, ROM loading, and simple graphics rendering for supported games.

## Features

- **6502 CPU emulation**  
  Implements core CPU instructions and memory addressing modes.

- **ROM loading**  
  Loads and parses iNES-format ROM files.

- **Basic memory bus**  
  Handles CPU memory mapping, including RAM and PRG ROM.

- **SDL2-based graphics**  
  Uses SDL2 for window management, rendering, and keyboard input.

- **Gamepad/Keyboard input**  
  Maps WASD keys to NES controller directions.

## Usage

### Requirements

- Rust (latest stable toolchain)
- SDL2 development libraries

### Build & Run

1. Install SDL2 (on Ubuntu: `sudo apt-get install libsdl2-dev`)
2. Build the emulator:

   ```sh
   cargo build --release
   ```

3. Place your NES ROM (e.g., `snake.nes`) in the project root directory. See [Easy 6502](https://skilldrick.github.io/easy6502/#snake) for an explanation of the 6502 language and the snake game.
4. Run the emulator:

   ```sh
   cargo run --release
   ```

   The emulator will load `snake.nes` by default.

### Controls

- **WASD**: Move up, left, down, right (NES D-pad)
- **Esc**: Quit

## Project Structure

- `src/main.rs` — Main entry point, SDL2 setup, rendering loop, and ROM loading.
- `src/cpu.rs` — 6502 CPU emulation logic.
- `src/bus.rs` — Memory bus, RAM mirroring, and ROM mapping.
- `src/cartridge.rs` — iNES ROM parsing and cartridge abstraction.
- `src/opcodes.rs` — Opcode definitions and decoding.

## Limitations & TODO

- PPU (graphics chip) and APU (audio chip) are not implemented (yet...).
- Only basic ROMs and a limited set of mappers are supported.
- No save states or debugging tools (yet...).
- Only keyboard input is supported.

---

> **Note:** `redNES` is a learning project and not intended for playing commercial NES games.
