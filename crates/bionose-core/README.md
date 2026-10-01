# bionose-core

[![Crates.io](https://img.shields.io/crates/v/bionose-core.svg)](https://crates.io/crates/bionose-core)
[![Documentation](https://docs.rs/bionose-core/badge.svg)](https://docs.rs/bionose-core)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](../../LICENSE-MIT)

Neuromorphic on-device olfactory engine inspired by *Drosophila melanogaster* antennal lobe divisive normalization and mushroom body sparse coding.

Designed strictly for `#![no_std]`, zero dynamic heap allocation, and deterministic sub-2-microsecond edge inference on resource-constrained microcontrollers (ESP32, ARM Cortex-M4/M7, RISC-V).

## Architecture

1. **Weber-Fechner Transduction:** Linearizes Langmuir-Hinshelwood sub-linear sensor adsorption kinetics.
2. **Antennal Lobe Divisive Normalization:** Subtractive lateral inhibition eliminates ambient common-mode humidity/weather baseline shifts.
3. **Adaptive Cosine Classifier:** Continual field calibration without catastrophic forgetting.
4. **Modbus RTU Telemetry:** Industrial zero-allocation framing for SCADA/PLC integration.

## Usage

```toml
[dependencies]
bionose-core = { version = "0.1.0", default-features = false }
```

```rust
use bionose_core::{AdaptiveNoseConfig, AdaptiveNoseEngine};

let config = AdaptiveNoseConfig::industrial_default();
let mut engine = AdaptiveNoseEngine::<16, 6>::new(&config);

// Raw resistance readings from 16 MOS sensor channels (Ohms)
let raw_readings = [12_500.0; 16];
let result = engine.infer(&raw_readings);

if !result.is_novel {
    println!("Detected Gas Class: {}, Confidence: {} bps", result.best_class + 1, result.confidence_basis_points);
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
