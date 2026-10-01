# Install & Hardware Integration Guide: BioNose-Edge

**Languages:** [English](INSTALL.md) (default) · [فارسی](INSTALL.fa.md)

This technical specification covers building, testing, packaging, and deploying `BioNose-Edge` across host operating systems (Linux, Windows, macOS) and embedded microcontroller architectures (ARM Cortex-M, RISC-V, and Espressif Xtensa).

---

## 1. Toolchain Prerequisites

| Component | Minimum Version | Functional Requirement |
| :--- | :---: | :--- |
| **Rust Toolchain** | `1.80+` | Compiling `bionose-core` (`#![no_std]`) and `bionose-cli` |
| **Host Target** | `x86_64` / `aarch64` | Running desktop benchmarks, stress test suites, and CLI tools |
| **Embedded Cross-Targets** | See Section 3 | Target compilation for bare-metal microcontrollers |
| **Git** | `2.30+` | Source control and submodule management |

---

## 2. Host Development & Verification Suite

### Step 1: Clone Repository
```sh
git clone https://github.com/Ali-Rashidi-80/bionose-edge.git
cd bionose-edge
```

### Step 2: Run Full Automated Verification Loop
```sh
# Format checking
cargo fmt --all -- --check

# Strict Clippy linter with zero warnings
cargo clippy --all-targets --all-features -- -D warnings

# Run all 34 unit, integration, and stress tests
cargo test --all --verbose

# Run adversarial fault-injection suite
cargo test -p bionose-core --test adversarial_stress_tests --verbose
```

### Step 3: Run the 36-Month UCI Benchmark Tournament
```sh
# Executes tournament across 13,910 physical gas measurements
cargo run --release -p bionose-cli
```

---

## 3. Microcontroller Hardware Targets Matrix

`bionose-core` is strictly `#![no_std]`, requires **zero dynamic heap allocation** (`alloc` is completely absent), and relies solely on `core` and `libm` (for software floating point).

### Comprehensive Target Compatibility Table

| Architecture Family | Rust Target Triple | Representative Silicon / Chips | Floating-Point Mode | RAM Footprint | Status |
| :--- | :--- | :--- | :---: | :---: | :---: |
| **ARM Cortex-M0 / M0+** | `thumbv6m-none-eabi` | Raspberry Pi RP2040, STM32G0/L0/F0, SAMD21 | Soft-float (`libm`) | < 1.5 KB | **Tier 1 Verified** |
| **ARM Cortex-M3** | `thumbv7m-none-eabi` | STM32F103 ("Blue Pill"), STM32L1, LPC1768 | Soft-float (`libm`) | < 1.5 KB | **Tier 1 Verified** |
| **ARM Cortex-M4 / M7** | `thumbv7em-none-eabi` | STM32F301, NXP Kinetis K20/K64 (non-FPU) | Soft-float (`libm`) | < 1.5 KB | **Tier 1 Verified** |
| **ARM Cortex-M4F / M7F** | `thumbv7em-none-eabihf` | STM32F401/411, STM32H7, Nordic nRF52840 | Hardware VFPv4/FPv5 | < 1.5 KB | **Tier 1 Verified** |
| **ARM Cortex-M33 / M55** | `thumbv8m.main-none-eabihf` | Nordic nRF5340, STM32H5, STM32U5, LPC55S69 | Hardware FPU + TrustZone | < 1.5 KB | **Tier 1 Verified** |
| **RISC-V 32-bit (IMC)** | `riscv32imc-unknown-none-elf` | Espressif ESP32-C3, WCH CH32V103/V203/V307 | Soft-float (`libm`) | < 1.5 KB | **Tier 1 Verified** |
| **RISC-V 32-bit (IMAC)** | `riscv32imac-unknown-none-elf` | Espressif ESP32-C6 (Wi-Fi 6/802.15.4), ESP32-H2 | Soft-float + Atomics | < 1.5 KB | **Tier 1 Verified** |
| **Espressif Xtensa** | `xtensa-esp32s3-none-elf` | ESP32, ESP32-S2, ESP32-S3 (AI Vector Ext) | Hardware/Vector | < 1.5 KB | **Verified via espup** |

---

## 4. Cross-Compilation Commands

### Installing Embedded Toolchains
```sh
rustup target add \
  thumbv6m-none-eabi \
  thumbv7m-none-eabi \
  thumbv7em-none-eabi \
  thumbv7em-none-eabihf \
  thumbv8m.main-none-eabihf \
  riscv32imc-unknown-none-elf \
  riscv32imac-unknown-none-elf
```

### Target Verification Commands

#### 1. Raspberry Pi RP2040 / ARM Cortex-M0+
```sh
cargo check -p bionose-core --no-default-features --target thumbv6m-none-eabi
```

#### 2. STM32F103 / ARM Cortex-M3
```sh
cargo check -p bionose-core --no-default-features --target thumbv7m-none-eabi
```

#### 3. STM32F4 / Nordic nRF52840 (BLE 5.0) / ARM Cortex-M4F
```sh
cargo check -p bionose-core --no-default-features --target thumbv7em-none-eabihf
```

#### 4. Nordic nRF5340 / STM32H5 / ARM Cortex-M33
```sh
cargo check -p bionose-core --no-default-features --target thumbv8m.main-none-eabihf
```

#### 5. Espressif ESP32-C3 (RISC-V)
```sh
cargo check -p bionose-core --no-default-features --target riscv32imc-unknown-none-elf
```

#### 6. Espressif ESP32-C6 / ESP32-H2 (RISC-V + Zigbee/Thread)
```sh
cargo check -p bionose-core --no-default-features --target riscv32imac-unknown-none-elf
```

#### 7. Espressif ESP32-S3 (Xtensa Dual-Core LX7)
Using the official `esp-rs` toolchain:
```sh
# Install espup toolchain manager
cargo install espup
espup install

# Build for ESP32-S3 bare-metal
cargo build -p bionose-core --target xtensa-esp32s3-none-elf --no-default-features --release
```

---

## 5. Architectural Reality Check: 8-bit MCUs (AVR / Arduino Uno)

> [!WARNING]
> **Adversarial Engineering Constraint:**
> While `bionose-core` does not prohibit compilation on 8-bit AVR microcontrollers (e.g. `avr-unknown-gnu-atmega328`), running on legacy 8-bit hardware is **strictly not recommended** for production:
> - **SRAM Scarcity:** An ATmega328P possesses only 2,048 bytes (2 KB) of total SRAM. `BioNoseEngine` requires ~1.2 KB of static state memory for sensor calibration, projection buffers, and baseline tracking, consuming >60% of total system memory.
> - **Floating-Point Penalties:** The 8-bit AVR architecture lacks a hardware FPU; software-emulated 32-bit IEEE-754 float operations require hundreds of machine cycles per operation, degrading inference latency from 1.8 microseconds (on a 240 MHz 32-bit MCU) to >15 milliseconds (on a 16 MHz 8-bit MCU).
> - **Production Recommendation:** Deploy `bionose-core` exclusively on **32-bit and 64-bit microcontrollers** (Cortex-M0+, Cortex-M4, Cortex-M33, RISC-V, or ESP32), where it consumes less than 1% of RAM and executes in under 2 microseconds.

---

## 6. Firmware Integration Guide

Add `bionose-core` to your embedded firmware's `Cargo.toml`:

```toml
[dependencies]
bionose-core = { version = "0.1.0", default-features = false }
```

### Minimal Bare-Metal Firmware Implementation

```rust
#![no_std]
#![no_main]

use bionose_core::{AdaptiveNoseConfig, AdaptiveNoseEngine, ModbusSlave};
use panic_halt as _; // Or your platform's panic handler

#[entry]
fn main() -> ! {
    // 1. Initialize engine with 16 sensors and 6 gas classes
    let config = AdaptiveNoseConfig::industrial_default();
    let mut engine = AdaptiveNoseEngine::<16, 6>::new(&config);

    // 2. Hardware loop
    loop {
        // Sample 16-channel sensor array via ADC (raw Ohms)
        let sensor_resistances: [f32; 16] = read_hardware_adc();

        // Sub-2 microsecond edge inference
        let inference = engine.infer(&sensor_resistances);

        // Encode industrial Modbus RTU telemetry
        let mut telemetry = engine.to_modbus_telemetry(&inference, 1, 18);
        let slave = ModbusSlave::new(1);

        // Transmit telemetry over RS485 / Modbus bus
        transmit_rs485(telemetry.as_slice());
    }
}
```

---

## 7. Automated Packaging & crates.io Validation

To verify that `bionose-core` is ready for packaging without errors:

```sh
# Perform full packaging validation
cargo package -p bionose-core
```
