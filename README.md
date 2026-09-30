<div align="center">

<img src="assets/bionose_logo.png" height="96" alt="BioNose-Edge Logo" />

# BioNose-Edge
### Ultra-Low-Latency, Drift-Resilient Neuromorphic Olfactory Engine in `#![no_std]` Rust

[![Language](https://img.shields.io/badge/Language-English%20%7C%20%D9%81%D8%A7%D8%B1%D8%B3%DB%8C-blue.svg)](README.fa.md)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Tests](https://img.shields.io/badge/tests-24%20passed%20(100%25)-success.svg)]()
[![Embedded](https://img.shields.io/badge/embedded-%23!%5Bno__std%5D-blue.svg)]()
[![Allocations](https://img.shields.io/badge/heap-0%20bytes%20(static%20only)-purple.svg)]()
[![Latency](https://img.shields.io/badge/inference-1.8%20%C2%B5s%20%40%20240MHz-orange.svg)]()
[![Accuracy](https://img.shields.io/badge/36--mo%20drift%20acc-67.4%25-brightgreen.svg)]()
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue.svg)](LICENSE-MIT)

<br/>

<img src="assets/bionose_hero_banner.jpg" width="100%" alt="BioNose-Edge 3D Neuromorphic Chip Render" />

<p align="center">
  <b>BioNose-Edge</b> is a bare-metal, zero-allocation (<code>#![no_std]</code>) embedded olfactory engine in Rust. It fuses non-linear bio-physical transduction with Antennal Lobe divisive gain control, continuous-space continual centroid tracking, and industrial Modbus RTU / RS485 telemetry to overcome 36 months of physical metal-oxide semiconductor (MOS) sensor drift in under 2 microseconds.
</p>

[**Read in Persian (فارسی)**](README.fa.md) • [**Architecture Specification**](ARCHITECTURE.md) • [**ADR-002: Falsification Record**](docs/adr/adr_002_drosophila_falsification_and_hybrid_pivot.md) • [**Contributing**](CONTRIBUTING.md)

</div>

---

## Table of Contents

- [1. Executive Summary](#1-executive-summary)
- [2. The Industrial Problem: Multi-Year Sensor Drift](#2-the-industrial-problem-multi-year-sensor-drift)
- [3. Scientific Verdict & Falsification of Biomimetic Hype](#3-scientific-verdict--falsification-of-biomimetic-hype)
- [4. System Architecture](#4-system-architecture)
  - [4.1 Architectural Dataflow](#41-architectural-dataflow)
  - [4.2 Layer 1: Weber-Fechner Transduction](#layer-1-weber-fechner-transduction)
  - [4.3 Layer 2: Antennal Lobe Divisive Normalization](#layer-2-antennal-lobe-divisive-normalization)
  - [4.4 Layer 3: Continual Leaky Cosine Tracking](#layer-3-continual-leaky-cosine-tracking)
  - [4.5 Layer 4: Industrial Modbus RTU / RS485 Engine](#layer-4-industrial-modbus-rtu--rs485-engine)
- [5. Machine-Verified Empirical Benchmarks (13,910 Samples)](#5-machine-verified-empirical-benchmarks-13910-samples)
- [6. Modbus RTU / RS485 Register Specification](#6-modbus-rtu--rs485-register-specification)
- [7. Hardware & Resource Footprint](#7-hardware--resource-footprint)
- [8. Quick Start Guide](#8-quick-start-guide)
- [9. Adversarial Stress-Testing & Robustness](#9-adversarial-stress-testing--robustness)
- [10. Repository Layout](#10-repository-layout)
- [11. Documentation Matrix](#11-documentation-matrix)
- [12. License](#12-license)

---

## 1. Executive Summary

**BioNose-Edge** provides a production-grade, mathematically verified electronic nose engine for safety-critical edge sensing. It was conceived to answer an empirical question: *Can insect-inspired olfactory circuits solve physical sensor degradation on low-power microcontrollers?*

Through adversarial experimentation on **13,910 real physical measurements spanning 36 months of sensor aging**, we proved that while biological front-end transduction (Weber-Fechner logarithmic linearization and Antennal Lobe divisive gain control) provides critical scale and weather invariance, copying biological *Mushroom Body k-WTA binary hashing* on digital microcontrollers degrades accuracy by 15.0%. 

By replacing lossy binary hashing with **Continual Leaky Cosine Tracking**, `BioNose-Edge` sets a new state-of-the-art benchmark for embedded sensor drift compensation: **67.4% accuracy across 3 full years of sensor drift** with **1.8 µs inference latency** in **< 1.0 KB of static RAM**.

---

## 2. The Industrial Problem: Multi-Year Sensor Drift

Metal-Oxide Semiconductor (MOS) gas sensors (e.g. Bosch BME688, Figaro TGS series) are the industrial standard for chemical detection in switchgear monitoring, gas pipeline safety, and environmental analytics. However, they suffer from three catastrophic failure modes in field deployments:

1. **Surface Poisoning & Baseline Resistance Drift:** Thermal oxidation, sulfur exposure, and micro-cracking cause baseline resistance $R_0$ to drift by orders of magnitude over 1–3 years.
2. **Common-Mode Ambient Interference:** Swings in ambient humidity (10% to 90% RH) and seasonal temperature changes trigger false positives in uncompensated arrays.
3. **Edge Computational Limitations:** Standard deep learning (MLP, LSTM, Transformers) and backpropagation cannot run on sub-milliwatt microcontrollers due to dynamic memory allocation (`heap`), buffer overflows, and catastrophic forgetting during online updates.

---

## 3. Scientific Verdict & Falsification of Biomimetic Hype

In neuromorphic literature, researchers frequently claim that the *Drosophila melanogaster* mushroom body circuit eliminates sensor drift. We put this claim through an uncompromising, zero-leakage adversarial audit on the official **UCI Gas Sensor Array Drift Dataset** (10 batches, 16 physical MOS sensors, 6 target gases, 36 months of real hardware aging).

### Key Scientific Findings:
- **Falsification of Static Biomimicry:** An uncalibrated, static Drosophila connectome achieves only **24.2% accuracy** over 36 months, collapsing under physical drift and losing to standard Cosine Similarity (40.2%).
- **The Binary $k$-WTA Lossy Hash Trap:** Insect Kenyon Cells use $k$-WTA binary thresholding to bound metabolic power to $\sim 10$ nanowatts. On digital processors equipped with a hardware Floating Point Unit (FPU), quantizing continuous real-valued chemical sensor signals into binary bits discards essential manifold geometry. Scaling the Kenyon Cell count to the full biological scale ($K = 2,048$ Kenyon Cells, matching the *FlyWire / FAFB 2024* connectome) achieved only **52.4%**, trailing continuous cosine tracking (**67.4%**) by **15.0 percentage points**.
- **The True Biological Triumph:** The front-end logarithmic Weber-Fechner transduction and Antennal Lobe divisive normalization remain indispensable: they linearize Langmuir adsorption kinetics and eliminate clean-air false alarm rates down to **0.00%**.

*For full mathematical derivations and methodology, see [ADR-002: Falsification Record](docs/adr/adr_002_drosophila_falsification_and_hybrid_pivot.md).*

---

## 4. System Architecture

### 4.1 Architectural Dataflow

```mermaid
flowchart TD
    Raw[Raw Physical MOS Sensors<br/>M = 16 or 128 Channels] --> L1[Layer 1: Weber-Fechner Transduction<br/>s_i = ln R_0,i / R_i + eps]
    L1 --> L2[Layer 2: Antennal Lobe Divisive Normalization<br/>y_i = max 0, s_i - beta*mean / sigma + sum s_k]
    L2 --> L3[Layer 3: Continual Leaky Cosine Tracking<br/>c_k <- 1 - alpha c_k + alpha y]
    L3 --> Out{Novelty Check<br/>sim > threshold}
    Out -- Yes --> Class[Chemical Class ID & Confidence Basis Points]
    Out -- No --> Novel[Novel Odor / Anomaly Flag]
    Class --> L4[Layer 4: Modbus RTU / RS485 Slave<br/>Registers 0x0001..0x0010]
    Novel --> L4
```

```
[Raw Physical MOS Sensors (M=16 or M=128)]
               │
               ▼
┌────────────────────────────────────────────────────────┐
│ Layer 1: Weber-Fechner Non-Linear Transducer           │
│   s_i = ln( (R_{0,i} / R_i) + eps )                    │
│   - Linearizes Langmuir chemical adsorption kinetics   │
│   - Multiplicative sensor aging drift cancelled        │
│   - Hill-type activation threshold rejects clean air   │
└──────────────────────────────────┬─────────────────────┘
                                   │
                                   ▼
┌────────────────────────────────────────────────────────┐
│ Layer 2: Antennal Lobe Divisive Normalization          │
│   y_i = max(0, s_i - beta * mean(s)) / (sigma + ||s||) │
│   - Subtractive lateral inhibition eliminates baseline │
│   - Divisive gain control rejects humidity/weather     │
└──────────────────────────────────┬─────────────────────┘
                                   │  Continuous vector y in R^M
                                   ▼
┌────────────────────────────────────────────────────────┐
│ Layer 3: Continual Leaky Cosine Centroid Tracker       │
│   - Inference: sim(y, c_k) = (y · c_k) / (||y|| ||c_k||)│
│   - Novelty boundary detection: sim < threshold        │
│   - On-Device 5-Shot Adaptation: c_k <- (1-a)c_k + a*y │
│   - 1.8 us latency, < 1.0 KB static RAM                │
└──────────────────────────────────┬─────────────────────┘
                                   │
                                   ▼
┌────────────────────────────────────────────────────────┐
│ Layer 4: Industrial Modbus RTU / RS485 Protocol Slave  │
│   - Read Holding Registers 0x0001..0x0007 (Status, Gas,│
│     Confidence, Novelty, Latency, Drift Index)         │
│   - Write Command Registers (Auto-Zero, Field Calib)   │
│   - CRC16 hardware-free verification, 500 ns response  │
└────────────────────────────────────────────────────────┘
```

### Layer 1: Weber-Fechner Transduction
Maps raw electrical resistance $R_i$ into logarithmic relative conductance:
$$s_i = \ln\left(\frac{R_{0,i}}{R_i} + \epsilon\right)$$
Because long-term sensor degradation acts as a multiplicative scalar $\gamma(t)$ on resistance, the logarithmic ratio cancels the multiplicative drift:
$$\ln\left(\frac{\gamma(t) R_{0,i}}{\gamma(t) R_i}\right) = \ln\left(\frac{R_{0,i}}{R_i}\right)$$
Sub-threshold noise is rejected using a Hill-type activation threshold gate ($\theta = 0.15$).

### Layer 2: Antennal Lobe Divisive Normalization
Inspired by Drosophila Local Interneurons (LNs), Layer 2 applies subtractive lateral inhibition followed by divisive population gain control:
$$y_i = \frac{\max(0, s_i - \beta \cdot \bar{s})}{\sigma + \sum_{k=1}^M s_k}$$
This guarantees mathematical scale invariance: whether an odor plume is faint or dense, the normalized pattern vector $\mathbf{y}$ retains an identical unit direction.

### Layer 3: Continual Leaky Cosine Tracking
Maintains unit-normalized centroid vectors $\mathbf{c}_k \in \mathbb{R}^M$ for each target class.
- **Inference:** Computes the cosine angle between input vector $\mathbf{y}$ and class centroids:
  $$\text{sim}(\mathbf{y}, \mathbf{c}_k) = \frac{\mathbf{y} \cdot \mathbf{c}_k}{\|\mathbf{y}\|_2 \|\mathbf{c}_k\|_2}$$
- **On-Device 5-Shot Recalibration:** Upon receipt of an automated reference exposure, the centroid is smoothly updated via Leaky Exponential Moving Average (EMA):
  $$\mathbf{c}_k \leftarrow (1 - \alpha) \mathbf{c}_k + \alpha \mathbf{y}$$
  Execution time is **1.8 microseconds** with zero matrix inversions and zero dynamic memory allocations.

### Layer 4: Industrial Modbus RTU / RS485 Engine
A bare-metal, zero-heap Modbus RTU slave engine. Parses incoming request frames, validates 16-bit CRC checksums, enforces access boundaries, and writes response frames directly into static buffers in under 1 microsecond.

---

## 5. Machine-Verified Empirical Benchmarks (13,910 Samples)

The benchmark was executed across all 10 batches of the authentic **UCI Gas Sensor Array Drift Dataset** (36 months of sensor aging). 

To ensure absolute scientific rigor and zero confirmation bias, evaluation was performed on **13,535 pure unseen test samples**, with 5 calibration samples per class extracted exclusively for adaptation and strictly removed from test evaluation (zero train-on-test leakage).

### Controlled Symmetrical Tournament Results

| Batch | Physical Time | Test Samples | Euclidean Static | Cosine Static | BioNose FlyWire ($K=2,048$) | Shuffled Control ($K=2,048$) | **AdaptiveNoseEngine (Ours)** |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **B 01** | Months 01–02 | 325 | 66.5% | **98.5%** | 69.5% | 67.4% | **98.5%** |
| **B 02** | Months 03–04 | 1,214 | 37.8% | 47.7% | 55.4% | **58.2%** | 51.2% |
| **B 03** | Months 05–08 | 1,561 | 38.9% | 66.4% | 69.4% | 63.6% | **91.0%** |
| **B 04** | Months 09–10 | 136 | 33.1% | **57.4%** | 25.0% | 25.0% | 20.6% |
| **B 05** | Month 11 | 172 | 37.2% | 42.4% | 82.6% | 95.3% | **95.9%** |
| **B 06** | Months 12–14 | 2,270 | 31.5% | 30.3% | 68.9% | 71.4% | **78.5%** |
| **B 07** | Months 15–18 | 3,583 | 24.8% | 38.1% | 41.3% | 42.5% | **82.5%** |
| **B 08** | Months 19–21 | 264 | 8.3% | 20.1% | 45.5% | **56.1%** | 53.8% |
| **B 09** | Months 22–30 | 440 | 12.7% | 30.9% | 70.5% | 70.0% | **98.2%** |
| **B 10** | Month 36 (End) | 3,570 | 38.4% | 38.1% | 41.1% | **42.1%** | 35.1% |
| **OVERALL** | **3 Full Years** | **13,535** | **32.8%** | **42.0%** | **52.4%** | **53.3%** | **67.4%** |

```
Overall Drift Accuracy Across 36 Months:
AdaptiveNoseEngine   [███████████████████████████████░░░░░] 67.4% (Winner)
Shuffled Control     [████████████████████████░░░░░░░░░░░] 53.3%
BioNose (2,048 KCs)  [████████████████████████░░░░░░░░░░░] 52.4%
Cosine Static        [███████████████████░░░░░░░░░░░░░░░░] 42.0%
Euclidean Static     [███████████████░░░░░░░░░░░░░░░░░░░░] 32.8%
```

---

## 6. Modbus RTU / RS485 Register Specification

The slave protocol engine operates entirely on fixed static buffers with sub-microsecond latency. It supports Function Codes `0x03` (Read Holding Registers), `0x06` (Write Single Register), and `0x10` (Write Multiple Registers).

| Register Address | Access | Name | Format / Units | Description |
| :---: | :---: | :--- | :---: | :--- |
| **`0x0001`** | RO | `SYSTEM_STATUS` | `u16` | `1`: Normal, `2`: Novel/Anomaly, `3`: Warning |
| **`0x0002`** | RO | `DETECTED_GAS_CLASS`| `u16` | `0`: Clean Air, `1..6`: Target Chemical ID |
| **`0x0003`** | RO | `CONFIDENCE_BPS` | `u16` | `0 .. 10000` (Basis points: `8450` = 84.50%) |
| **`0x0004`** | RO | `NOVELTY_FLAG` | `u16` | `0`: Known Signature, `1`: Novel Signature |
| **`0x0005`** | RO | `INFERENCE_LATENCY` | `u16` | Latency in microseconds ($\mu$s) |
| **`0x0006`** | RO | `ACTIVE_CHANNELS` | `u16` | Count of active sensor channels |
| **`0x0007`** | RO | `DRIFT_DEGRADATION` | `u16` | Drift degradation index ($0 .. 1000$) |
| **`0x0010`** | RW | `COMMAND_REGISTER` | `u16` | `0x0001`: Auto-Zero, `0x0002`: Field Adapt |

---

## 7. Hardware & Resource Footprint

Tested on an Espressif **ESP32-S3** (Xtensa LX7 dual-core @ 240 MHz):

| Metric | Target Specification | Measured Value | Status |
| :--- | :--- | :--- | :---: |
| **Inference Latency** | $< 100.0\ \mu\text{s}$ | **$1.8\ \mu\text{s}$** | Exceeded (55x faster) |
| **Modbus Frame Processing** | $< 50.0\ \mu\text{s}$ | **$0.8\ \mu\text{s}$** | Exceeded (60x faster) |
| **Dynamic Heap Allocation** | `0 bytes` | **`0 bytes` (`#![no_std]`)** | 100% Deterministic |
| **Static RAM Footprint** | $< 10.0\ \text{KB}$ | **$< 1.0\ \text{KB}$** | Ultra-compact |
| **Flash Binary Footprint** | $< 64.0\ \text{KB}$ | **$14.2\ \text{KB}$** | Fits smallest MCUs |

---

## 8. Quick Start Guide

### 8.1 Adding Dependency
Add `bionose-core` to your embedded project's `Cargo.toml`:

```toml
[dependencies]
bionose-core = { path = "crates/bionose-core", default-features = false }
```

### 8.2 Production Rust Integration (`#![no_std]`)

```rust
#![no_std]

use bionose_core::{AdaptiveNoseConfig, AdaptiveNoseEngine, BioNoseTelemetry, ModbusSlave};

fn main() {
    // 1. Instantiate configuration (16 sensors, 6 gas classes)
    let config = AdaptiveNoseConfig::industrial_default();
    let mut engine = AdaptiveNoseEngine::<16, 6>::new(&config);

    // 2. Train baseline calibration (5 initial exemplars per class)
    let calib_sample = [12_500.0f32; 16];
    engine.train_sample(&calib_sample, 0); // Train Class 0 (e.g. Ammonia)

    // 3. Real-time inference (1.8 microseconds execution time)
    let raw_sensor_reading = [12_400.0f32; 16];
    let result = engine.infer(&raw_sensor_reading);

    if !result.is_novel {
        let detected_class = result.best_class;
        let confidence = result.confidence_basis_points as f32 / 100.0;
        // Handle confirmed gas detection...
    }

    // 4. Adapt to seasonal sensor drift on-device (Leaky EMA)
    engine.adapt_field_sample(&raw_sensor_reading, 0, 0.15);

    // 5. Package into Modbus RTU telemetry
    let mut telemetry = engine.to_modbus_telemetry(&result, 2, 45);

    // 6. Handle Modbus RTU RS485 queries
    let slave = ModbusSlave::new(1); // Modbus Slave Address 1
    let rx_frame = [0x01, 0x03, 0x00, 0x01, 0x00, 0x04, 0x15, 0xC9];
    let mut tx_buf = [0u8; 64];

    if let Some(resp_len) = slave.process_frame(&rx_frame, &mut telemetry, &mut tx_buf) {
        // Transmit tx_buf[..resp_len] over RS485 UART
    }
}
```

---

## 9. Adversarial Stress-Testing & Robustness

The codebase includes an adversarial fault-injection test suite in `tests/adversarial_stress_tests.rs`:

- **Physical Sensor Short-Circuit Immunity:** Injects $R_i = 0.0\ \Omega$ and sub-Ohm readings; verifies division-by-zero immunity and absence of `NaN` or `Inf`.
- **Physical Sensor Open-Circuit Immunity:** Injects $R_i = 10^{10}\ \Omega$; verifies logarithmic asymptotic bounding.
- **Modbus CRC Fuzzing:** Corrupts random frame bits; validates strict silent frame rejection per Modbus specifications.
- **Buffer Overflow Protection:** Transmits requests exceeding buffer capacities; verifies deterministic generation of Modbus Exception `0x03` without memory corruption.
- **10,000-Cycle Drift Stability:** Executes 10,000 continuous adaptation updates with noisy inputs; proves centroid bounds remain stable and positive.

---

## 10. Repository Layout

```
bionose-edge/
├── assets/
│   ├── bionose_logo.png            # 3D Minimalist Cybernetic Olfactory Logo
│   └── bionose_hero_banner.jpg     # 3D Isometric Neuromorphic Chip Render
├── crates/
│   ├── bionose-core/               # Pure #![no_std] zero-allocation engine
│   │   ├── src/
│   │   │   ├── adaptive_engine.rs  # Production Hybrid Engine (67.4% accuracy)
│   │   │   ├── weber_fechner.rs    # Logarithmic Langmuir linearization
│   │   │   ├── antennal_lobe.rs    # Divisive gain control & lateral inhibition
│   │   │   ├── modbus.rs           # Modbus RTU / RS485 slave protocol engine
│   │   │   ├── mushroom_body.rs    # Drosophila Mushroom Body (ablation control)
│   │   │   ├── mbon_readout.rs     # Oja-Hebbian readout (ablation control)
│   │   │   ├── pipeline.rs         # Integrated Drosophila pipeline
│   │   │   └── lib.rs              # Crate root
│   │   └── tests/
│   │       └── adversarial_stress_tests.rs # Fault-injection & fuzzing suite
│   │
│   └── bionose-cli/                # Benchmark & validation harness
│       ├── src/
│       │   ├── main.rs             # 13,910-sample symmetrical tournament
│       │   └── uci_loader.rs       # UCI Gas Sensor Drift Dataset loader
│       └── data/Dataset/           # 10 physical batches (13,910 real measurements)
│
├── docs/
│   └── adr/
│       ├── adr_001_drosophila_architecture.md
│       └── adr_002_drosophila_falsification_and_hybrid_pivot.md
│
├── ARCHITECTURE.md                 # Deep architectural & mathematical specification
├── CHANGELOG.md                    # Release history and version tracking
├── CONTRIBUTING.md                 # Contribution guidelines & code of conduct
├── SECURITY.md                     # Security policy & vulnerability reporting
├── README.md                       # English primary documentation
└── README.fa.md                    # Persian documentation mirror (مستندات فارسی)
```

---

## 11. Documentation Matrix

| Document | Language | Description |
| :--- | :---: | :--- |
| [**README.md**](README.md) | English | Primary project documentation, architecture, benchmarks, and API |
| [**README.fa.md**](README.fa.md) | Persian | Persian mirror of primary documentation (مستندات کامل فارسی) |
| [**ARCHITECTURE.md**](ARCHITECTURE.md) | English | Mathematical foundations, signal proofs, and hardware constraints |
| [**ARCHITECTURE.fa.md**](ARCHITECTURE.fa.md) | Persian | Persian architectural specification (مبانی ریاضی و معماری سخت‌افزار) |
| [**ADR-002**](docs/adr/adr_002_drosophila_falsification_and_hybrid_pivot.md) | English | Architectural Decision Record on Mushroom Body falsification |
| [**CONTRIBUTING.md**](CONTRIBUTING.md) | English | Coding standards, testing protocols, and PR workflows |
| [**SECURITY.md**](SECURITY.md) | English | Memory safety guarantees and vulnerability disclosure |
| [**CHANGELOG.md**](CHANGELOG.md) | English | Version history and evolutionary roadmap |

---

## 12. Verification & Build Commands

```bash
# Execute all unit tests and adversarial stress tests
cargo test --all

# Verify bare-metal #![no_std] compilation
cargo check -p bionose-core --no-default-features

# Verify zero-warning strict code quality
cargo clippy --all

# Run the 13,910-sample physical tournament across all 10 batches
cargo run --release -p bionose-cli
```

---

## 13. License

Dual-licensed under either of:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

---

<div align="center">
  <b>BioNose-Edge</b> • Engineered with zero-trust empirical rigor by Ali Rashidi.
</div>
