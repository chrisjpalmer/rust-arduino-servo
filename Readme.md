# Rust Arduino Servo

This project implements project 5 of the arduino book (mood cue).

## Getting Started

1. clone this repo
2. make sure you have the avr compiler and ravedude installed, checkout
   https://github.com/Rahix/avr-hal for more info. P.S. on a mac you can install
   the avr compiler with `brew` so just have a google.
3. With your arduino plugged in, run `cargo run --release`. The dev build
   doesn't work due to some issues with compiler builtins.

## How I set it up

To set it up, I tried to replicate as many settings as possible from
https://github.com/Rahix/avr-hal. This meant setting up the following files
correctly:

1. .cargo/config.toml (copied from https://github.com/Rahix/avr-hal)
2. rust-toolchain.toml (copied from https://github.com/Rahix/avr-hal)
3. cargo.toml (copied some stuff from https://github.com/Rahix/avr-hal)
4. avr-atmega328p.json (copied from https://github.com/Rahix/avr-hal)
