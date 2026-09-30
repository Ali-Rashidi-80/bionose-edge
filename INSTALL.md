# Install & Integration Guide: BioNose-Edge

**Languages:** [English](INSTALL.md) (default) · [فارسی](INSTALL.fa.md)

This guide covers building, testing, and integrating `BioNose-Edge` across host operating systems (Linux/Windows/macOS) and embedded microcontrollers (ESP32-S3, ARM Cortex-M, and RISC-V).

---

## 1. Prerequisites

| Tool | Minimum Version | Purpose |
| :--- | :---: | :--- |
| **Rust Toolchain** | `1.80+` | Compiling `bionose-core` and running tests |
| **Cargo Target (Host)** | `x86_64` / `aarch64` | Running CLI benchmarks on the UCI dataset |
| **Embedded Cross-Target** | `thumbv7em-none-eabihf` / `riscv32imc-unknown-none-elf` | Bare-metal MCU compilation (`#![no_std]`) |
| **Git** | `2.30+` | Repository cloning and version tracking |

---

## 2. Host Development & Verification

### Step 1: Clone Repository
```sh
git clone https://github.com/Ali-Rashidi/BioNose-Edge.git
cd BioNose-Edge
```

### Step 2: Run Automated Unit Tests & Stress Suite
```sh
# Run all 24 unit and adversarial stress tests
cargo test --all

# Run the 8 adversarial fault-injection tests specifically
cargo test --test adversarial_stress_tests
```

### Step 3: Run the 36-Month UCI Benchmark Tournament
```sh
# Executes tournament across 13,910 real physical measurements
cargo run --release -p bionose-cli
```

---

## 3. Embedded MCU Cross-Compilation (`#![no_std]`)

The core library `bionose-core` is strictly `#![no_std]` and requires zero dynamic heap allocation.

### Target A: ARM Cortex-M4 / Cortex-M7 (with hardware FPU)
```sh
# 1. Install target toolchain
rustup target add thumbv7em-none-eabihf

# 2. Check bare-metal compilation with zero default features
cargo check -p bionose-core --target thumbv7em-none-eabihf --no-default-features
```

### Target B: RISC-V 32-bit (e.g. ESP32-C3)
```sh
# 1. Install target toolchain
rustup target add riscv32imc-unknown-none-elf

# 2. Verify compilation
cargo check -p bionose-core --target riscv32imc-unknown-none-elf --no-default-features
```

### Target C: Espressif ESP32-S3 (Xtensa Dual-Core)
Using the official `esp-rs` toolchain:
```sh
# Install espup and toolchain
cargo install espup
espup install

# Build for Xtensa target
cargo build -p bionose-core --target xtensa-esp32s3-none-elf --no-default-features --release
```

---

## 4. Integrating into Your Embedded Firmware

Add `bionose-core` to your firmware's `Cargo.toml`:

```toml
[dependencies]
bionose-core = { git = "https://github.com/Ali-Rashidi/BioNose-Edge.git", default-features = false }
```

### Minimal Bare-Metal Example

```rust
#![no_std]
use bionose_core::{AdaptiveNoseConfig, AdaptiveNoseEngine, ModbusSlave};

// 1. Initialize engine with 16 sensors and 6 gas classes
let config = AdaptiveNoseConfig::industrial_default();
let mut engine = AdaptiveNoseEngine::<16, 6>::new(&config);

// 2. Read sensor array into fixed array
let sensor_resistances: [f32; 16] = read_hardware_adc();

// 3. Sub-2us inference
let inference = engine.infer(&sensor_resistances);

// 4. Modbus RTU telemetry
let mut telemetry = engine.to_modbus_telemetry(&inference, 1, 18);
let slave = ModbusSlave::new(1);
```

---

## 5. Acceptance Verification Checklist

Confirm all gates pass before deploying firmware to physical hardware:

```sh
cargo test --all
cargo check -p bionose-core --no-default-features
cargo clippy --all -- -D warnings
cargo fmt --all -- --check
```
