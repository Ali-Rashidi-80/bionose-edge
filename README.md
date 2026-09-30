# BioNose-Edge: Drift-Resilient Embedded Olfactory Engine

An ultra-low-latency, zero-allocation (`#![no_std]`) embedded olfactory engine in Rust. Engineered for industrial electronic noses, gas leak detection, and continuous air quality monitoring on microcontrollers (ESP32-S3, ARM Cortex-M, RISC-V).

---

## 1. Executive Architecture Summary

`BioNose-Edge` combines neurobiological sensor front-ends with high-precision continuous vector tracking and industrial Modbus RTU telemetry:

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

---

## 2. Empirical Verification on 13,910 Real Physical Measurements

The engine was evaluated on the official **UCI Gas Sensor Array Drift Dataset** (13,910 real physical measurements spanning 36 months of physical sensor aging across 16 physical MOS sensors and 6 target gases).

### Symmetrical Head-to-Head Benchmark (Zero Data Leakage)

Evaluating **13,535 pure unseen test samples** with strictly disjoint 5-shot calibration per batch:

| Architecture / Algorithm | Steady-State ($M=16$) | Full Dynamic Kinetics ($M=128$) | Inference Latency | RAM Footprint |
| :--- | :--- | :--- | :--- | :--- |
| **Euclidean Static** (Frozen) | 31.8% | 32.8% | 1.1 $\mu$s | 0.8 KB |
| **Cosine Static** (Frozen) | 39.7% | 42.0% | 1.2 $\mu$s | 0.8 KB |
| **BioNose FlyWire Scale** ($K=2,048$, $k$-WTA) | 41.2% | 52.4% | 38.6 $\mu$s | 48.0 KB |
| **Shuffled Control Graph** ($K=2,048$) | 45.2% | 53.3% | 38.6 $\mu$s | 48.0 KB |
| **Euclidean Continual** (Leaky EMA) | 50.3% | 50.6% | 1.2 $\mu$s | 0.8 KB |
| **AdaptiveNoseEngine** (Cosine Continual) | **61.1%** | **67.4%** | **1.8 $\mu$s** | **< 1.0 KB** |

### Key Scientific Findings (Falsification of Biomimetic Vanity)
1. **The Mushroom Body $k$-WTA is a Lossy Hash:** Silencing 95% of neurons into a binary bitmask serves a biological purpose ($\sim 10$ nW metabolic survival), but on a digital 32-bit hardware FPU (ESP32-S3), converting real-valued sensor signals into binary bitmasks degrades accuracy by **15.0 percentage points**.
2. **Continuous Vector Geometry Wins:** Unit-normalized continuous directional tracking preserves the complete orientation of the chemical odor manifold as sensors age, achieving **67.4% accuracy across 3 full years of sensor drift**.
3. **Sensor Front-End Verified:** Weber-Fechner non-linear logarithmic transduction and Antennal Lobe divisive normalization remain indispensable for physical sensor linearization and zero clean-air false alarms.

---

## 3. Industrial Modbus RTU / RS485 Register Map

The built-in Modbus RTU slave module requires zero dynamic memory allocation and processes query frames in under 1 microsecond:

| Address | Register Type | Parameter | Range / Units |
| :--- | :--- | :--- | :--- |
| **`0x0001`** | Holding (RO) | System Status | `1`: Normal, `2`: Novel/Anomaly, `3`: Warning |
| **`0x0002`** | Holding (RO) | Detected Gas Class | `0`: Clean Air, `1..6`: Target Gas ID |
| **`0x0003`** | Holding (RO) | Confidence Score | `0 .. 10,000` (Basis Points: 0.00% .. 100.00%) |
| **`0x0004`** | Holding (RO) | Novelty / Anomaly Flag | `0`: Recognized Gas, `1`: Novel Odor |
| **`0x0005`** | Holding (RO) | Inference Latency | Microseconds ($\mu$s) |
| **`0x0006`** | Holding (RO) | Active Sensor Channels | Count ($0 .. M$) |
| **`0x0007`** | Holding (RO) | Drift Degradation Index | $0 .. 1,000$ |
| **`0x0010`** | Holding (RW) | Command Register | `0x0001`: Trigger Auto-Zero Baseline Update |

---

## 4. Usage Example (`#![no_std]`)

Add `bionose-core` to your `Cargo.toml`:

```toml
[dependencies]
bionose-core = { path = "crates/bionose-core", default-features = false }
```

In your embedded application:

```rust
use bionose_core::{AdaptiveNoseConfig, AdaptiveNoseEngine, ModbusSlave, BioNoseTelemetry};

// 1. Configure for 16 physical MOS sensor channels and 6 chemical classes
let config = AdaptiveNoseConfig::industrial_default();
let mut engine = AdaptiveNoseEngine::<16, 6>::new(&config);

// 2. Perform field calibration (e.g. 5 reference exposures)
let calibration_sample = [12_500.0; 16];
engine.train_sample(&calibration_sample, 0); // Train Class 0

// 3. Ultra-low-latency real-time inference (1.8 microseconds on ESP32-S3)
let raw_sensor_readings = [12_450.0; 16];
let result = engine.infer(&raw_sensor_readings);

if !result.is_novel {
    // Gas identified with high confidence
    let gas_class = result.best_class;
    let confidence_pct = result.confidence_basis_points as f32 / 100.0;
}

// 4. Adapt in the field to slow seasonal drift (Leaky EMA update)
engine.adapt_field_sample(&raw_sensor_readings, 0, 0.15);

// 5. Export telemetry directly to industrial Modbus RTU frame
let telemetry = engine.to_modbus_telemetry(&result, 2, 45);
```

---

## 5. Repository Structure

```
bionose-edge/
├── crates/
│   ├── bionose-core/              # Pure #![no_std] zero-allocation engine
│   │   ├── src/
│   │   │   ├── adaptive_engine.rs # Production Hybrid Engine (Weber-Fechner + Antennal Lobe + Cosine Centroids)
│   │   │   ├── weber_fechner.rs   # Logarithmic Langmuir linearization
│   │   │   ├── antennal_lobe.rs   # Divisive gain control & lateral inhibition
│   │   │   ├── modbus.rs          # Industrial Modbus RTU / RS485 slave handler
│   │   │   ├── mushroom_body.rs   # Drosophila Mushroom Body (ablation reference)
│   │   │   ├── mbon_readout.rs    # Oja-Hebbian readout (ablation reference)
│   │   │   └── lib.rs             # Crate root
│   │   └── tests/
│   │       └── adversarial_stress_tests.rs # Fault-injection & fuzzing suite
│   │
│   └── bionose-cli/               # Physical benchmark harness
│       ├── src/
│       │   ├── main.rs            # Symmetrical benchmark tournament
│       │   └── uci_loader.rs      # Authentic UCI 10-batch dataset loader
│       └── data/Dataset/          # 10 batch files (13,910 real physical samples)
│
└── README.md                      # Architecture, benchmarks & integration guide
```

---

## 6. Verification & Build Commands

```bash
# Run all unit and adversarial fault-injection tests
cargo test --all

# Verify pure bare-metal #![no_std] compatibility
cargo check -p bionose-core --no-default-features

# Run zero-warning code quality linter
cargo clippy --all

# Run the 13,910-sample physical tournament
cargo run --release -p bionose-cli
```

---

## 7. License

Dual-licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))
