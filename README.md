# Lobste.rs

Lobste.rs is a minimal GameBoy emulator written in Rust. This is a personal project of mine that aims to learn and explore Rust's features. Any feedback is welcome.

## Current State

Other features of the emulator will be added here, I just haven't figured them out yet :P.

### CPU

The fetch, decode and execute logic is (hopefully) concluded, most logic is present in [cpu.rs](/src/cpu.rs).

Currently, I have only tested [Blargg's test 01](https://github.com/retrio/gb-test-roms/tree/master/cpu_instrs/individual), which does pass.

### PPU

Yet to implement

A test file using SDL2 bindings for Rust is present in [ppu.rs](/src/ppu.rs).

### Audio

Yet to implement

## Resources

Most of the information I got to develop this gameboy from [Pac Docs](https://gbdev.io/pandocs/).

Will also be using [Blargg's tests](https://github.com/retrio/gb-test-roms) to test the emulator's features.

[Gameboy doctor](https://github.com/robert/gameboy-doctor) is a really useful script that compares a control output with the one from yours.
