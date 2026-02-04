# Drone

[![Check](https://github.com/mrwatts88/drone/actions/workflows/check.yml/badge.svg)](https://github.com/mrwatts88/drone/actions/workflows/check.yml)
[![Format](https://github.com/mrwatts88/drone/actions/workflows/fmt.yml/badge.svg)](https://github.com/mrwatts88/drone/actions/workflows/fmt.yml)
[![Clippy](https://github.com/mrwatts88/drone/actions/workflows/clippy.yml/badge.svg)](https://github.com/mrwatts88/drone/actions/workflows/clippy.yml)

A from-scratch quadcopter where the goal is full understanding and ownership of every layer: mechanical design, electronics, real-time firmware, and control theory. Nothing is treated as a black box.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Host (Phone/Computer)                                      │
│  - Sends setpoints (throttle, roll, pitch, yaw)             │
│  - Receives telemetry                                       │
│  - Tuning and visualization                                 │
│  - Non-real-time, soft deadlines                            │
└─────────────────────────┬───────────────────────────────────┘
                          │ Custom protocol
┌─────────────────────────▼───────────────────────────────────┐
│  Flight Controller (STM32F446)                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ ISRs (minimal work, flag events)                       │ │
│  └──────────────────────┬─────────────────────────────────┘ │
│  ┌──────────────────────▼─────────────────────────────────┐ │
│  │ Sensor Acquisition (IMU reads, isolated)               │ │
│  └──────────────────────┬─────────────────────────────────┘ │
│  ┌──────────────────────▼─────────────────────────────────┐ │
│  │ State Sharing (sequence locks, explicit concurrency)   │ │
│  └──────────────────────┬─────────────────────────────────┘ │
│  ┌──────────────────────▼─────────────────────────────────┐ │
│  │ Control Logic (PID on validated state snapshots)       │ │
│  └──────────────────────┬─────────────────────────────────┘ │
│  ┌──────────────────────▼─────────────────────────────────┐ │
│  │ Motor Mixing (pure transformation)                     │ │
│  └──────────────────────┬─────────────────────────────────┘ │
└─────────────────────────┼───────────────────────────────────┘
                          │ PWM/DShot
┌─────────────────────────▼───────────────────────────────────┐
│  ESCs → Motors                                              │
└─────────────────────────────────────────────────────────────┘
```

## Mechanical

The frame is fully parametric CAD. Core parts:

- **Modular arms** — bolt-on, replaceable
- **Central plate** — holds electronics, defined by mounting holes
- **C-profile landing legs** — integrated brackets

Design principles:

- Interfaces defined by hole patterns and thickness constraints
- Parts iterate independently
- Strength from geometry (wide pads, thinner beams, fillets), not excess material
- Everything bolted, symmetric, manufacturable

## Electronics

Deliberately minimal and explicit. No off-the-shelf flight controller PCB.

| Component | Purpose |
|-----------|---------|
| STM32F446 Nucleo | Real-time brain |
| IMU (MPU6050/ICM-42688) | Orientation sensing |
| ESCs (4x) | Motor control |
| Power distribution | XT60, appropriate gauge wire |
| Smoke stopper | Current-limited testing |

Power, connectors, wire gauges, and soldering are part of the learning surface.

## Firmware

Written in Rust. Structured like a small real-time system, not an Arduino sketch.

### Control

PID loops are implemented from scratch. First-class concerns:

- Timing and update rates
- Stale read detection
- ISR safety
- Inspectable, debuggable control path

The intent is not just "it flies," but "the control path is understandable."

## Host Layer

A non-real-time layer (computer or phone) sits above the microcontroller:

- Sends setpoints: throttle, roll, pitch, yaw
- Receives telemetry
- Provides tuning interface and visualization
- Custom protocol (yours to define)

The separation between hard real-time physics and soft real-time UI/networking is intentional and strict.

## Development Setup

### Prerequisites

See GETTING_STARTED.md

### Build and flash

```bash
cargo run --release
```

