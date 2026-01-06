# Snake Terminal

[<img src="https://github.com/ratatui.png" align="right" width="100">](https://ratatui.rs)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

A terminal screensaver featuring an autonomous snake that hunts for food using pathfinding algorithms.

## Description

Snake Terminal is a visual terminal application where a green snake automatically navigates to consume red food pellets. Built with Ratatui, it provides an aesthetic terminal interface suitable for entertainment or as a live screensaver.

The snake uses BFS pathfinding to intelligently navigate around obstacles, creating smooth and realistic movement patterns.

## Features

- Autonomous snake with intelligent pathfinding
- Clean terminal UI with minimal design
- Configurable max score for auto-reset
- Real-time score tracking
- Smooth animations at 80ms tick rate

## Dependencies

- **ratatui** `0.26` - Terminal UI framework
- **crossterm** `0.27` - Terminal manipulation
- **rand** `0.8` - Random number generation

## Installation

```bash
git clone https://github.com/traitimtrongvag/predator-snake-rs.git
cd predator-snake-rs
cargo build --release
```

## Usage

Run with default settings (max score: 50):
```bash
cargo run --release
```

Run with custom max score:
```bash
cargo run --release -- 100
```

Press `q` to quit.

## Configuration

Speed can be adjusted in `src/main.rs`:
```rust
let tick_rate = Duration::from_millis(80); // Lower = faster
```
## Demo

![Snake Demo](https://github.com/traitimtrongvag/predator-snake-rs/raw/main/Example/example.gif)

## License

MIT
