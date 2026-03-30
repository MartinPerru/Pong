# Pong

A simple Pong game built with Rust and [macroquad](https://macroquad.rs/).

## Prerequisites

### Linux

```bash
# Ubuntu / Debian
sudo apt install libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# Arch Linux
sudo pacman -S libx11 libxi mesa alsa-lib
```

### macOS

No extra dependencies needed. Just make sure you have Xcode Command Line Tools:

```bash
xcode-select --install
```

### Rust (all platforms)

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Run

```bash
cargo run
```
Player Movements

W - UP

S - DOWN
