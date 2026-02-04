# STM32 Rust Development Setup (Linux)

A minimal guide to getting Rust running on STM32F4 microcontrollers on Linux.

## Hardware

- STM32F446RE Nucleo board (or similar STM32F4)
- USB cable for ST-LINK connection

## Prerequisites

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Add the ARM Cortex-M target

For STM32F4 (Cortex-M4F with hardware FPU):

```bash
rustup target add thumbv7em-none-eabihf
```

Other common targets:
- `thumbv6m-none-eabi` - Cortex-M0, M0+
- `thumbv7m-none-eabi` - Cortex-M3
- `thumbv7em-none-eabi` - Cortex-M4, M7 (no FPU)
- `thumbv7em-none-eabihf` - Cortex-M4F, M7F (with FPU)

### 3. Install probe-rs

This handles flashing and debugging via ST-LINK:

```bash
cargo install probe-rs-tools
```

### 4. Set up udev rules (for non-root ST-LINK access)

Create `/etc/udev/rules.d/69-probe-rs.rules`:

```
# ST-LINK V2
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", MODE="660", GROUP="plugdev", TAG+="uaccess"

# ST-LINK V2-1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", MODE="660", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3752", MODE="660", GROUP="plugdev", TAG+="uaccess"

# ST-LINK V3
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374d", MODE="660", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374e", MODE="660", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374f", MODE="660", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3753", MODE="660", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3754", MODE="660", GROUP="plugdev", TAG+="uaccess"
```

Then reload:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Make sure your user is in the `plugdev` group:

```bash
sudo usermod -aG plugdev $USER
```

Log out and back in for group changes to take effect.

## Project Structure

```
drone/
├── .cargo/
│   └── config.toml    # Build target and runner config
├── src/
│   └── main.rs        # Your code
├── Cargo.toml         # Dependencies
└── memory.x           # Memory layout for linker
```

## Configuration Files

### `.cargo/config.toml`

Sets the default target, flash runner, and linker script:

```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32F446RETx"
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

### `Cargo.toml`

```toml
[package]
name = "drone"
version = "0.1.0"
edition = "2024"

[dependencies]
embedded-hal = "1.0"
nb = "1"
cortex-m = "0.7"
cortex-m-rt = "0.7"
# Panic behaviour, see https://crates.io/keywords/panic-impl for alternatives
panic-halt = "1.0"

[dependencies.stm32f4xx-hal]
version = "0.23.0"
features = ["stm32f446"] # replace the model of your microcontroller here
                         # and add other required features
```

### `memory.x`

Defines the memory layout for your specific chip. This is **required** by `cortex-m-rt`:

```
MEMORY
{
  /* NOTE K = KiB = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}

/* Stack grows downward from the top of RAM */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
```

Check your chip's datasheet for correct FLASH and RAM sizes.

## Build and Flash

Build only:

```bash
cargo build --release
```

Build and flash to connected board:

```bash
cargo run --release
```

The `runner` in `.cargo/config.toml` makes `cargo run` automatically flash via probe-rs.

## Troubleshooting

### "no probe was found"

- Check USB connection
- Verify udev rules are loaded
- Make sure you're in the `plugdev` group
- Try `probe-rs list` to see detected probes

### "Error: chip not found"

Find your chip's exact name:

```bash
probe-rs chip list | grep -i stm32f4
```

Update the `--chip` argument in `.cargo/config.toml`.

### Linker errors about memory

- Verify `memory.x` exists in project root
- Check FLASH/RAM sizes match your chip
- Ensure `-Tlink.x` is in rustflags

## Resources

- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [probe-rs](https://probe.rs/)
- [stm32f4xx-hal docs](https://docs.rs/stm32f4xx-hal)
- [cortex-m-rt](https://docs.rs/cortex-m-rt)
