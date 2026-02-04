# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build and flash to device (requires probe-rs and connected ST-LINK)
cargo run --release

# Build only (debug)
cargo build

# Build only (release)
cargo build --release

# Check compilation without building
cargo check

# Format code
cargo fmt

# Run clippy lints
cargo clippy -- -D warnings
```

Note: The target (`thumbv7em-none-eabihf`) is configured in `.cargo/config.toml`, so you don't need to specify it explicitly.

## Architecture

This is bare-metal embedded Rust firmware for a quadcopter flight controller running on an STM32F446RE (Cortex-M4F).

### Runtime Environment

- `#![no_std]` - no standard library
- `#![no_main]` - custom entry point via `cortex-m-rt`
- `panic-halt` - halts CPU on panic
- Hardware FPU enabled (`thumbv7em-none-eabihf` target)

### Layered Control Flow

```
ISRs (minimal work, flag events)
    ↓
Sensor Acquisition (IMU reads)
    ↓
State Sharing (sequence locks for concurrency)
    ↓
Control Logic (PID on validated state snapshots)
    ↓
Motor Mixing (pure transformation)
    ↓
PWM/DShot output to ESCs
```

### Concurrency Pattern

ISRs access shared state via `Mutex<RefCell<Option<T>>>` from `cortex_m::interrupt`. Critical sections use `cortex_m::interrupt::free()`. ISRs should do minimal work—just read/flag, with processing deferred to the main loop.

### Hardware Configuration

- **MCU:** STM32F446RE Nucleo
- **Clock:** 48 MHz (HSI)
- **Debug output:** RTT via `rprintln!()` (no UART needed for debug)
- **USART2:** PA2 (TX), PA3 (RX) - connected to ST-LINK virtual COM at 115200 baud

### Memory Layout

Defined in `memory.x`:
- FLASH: 0x08000000, 512K
- RAM: 0x20000000, 128K
