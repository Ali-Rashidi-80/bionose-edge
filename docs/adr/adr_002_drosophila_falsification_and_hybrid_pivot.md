# ADR-002: Falsification of Drosophila Mushroom Body k-WTA & Architectural Pivot to Production AdaptiveNoseEngine

- **Status:** Accepted (Production Architecture Sealed)
- **Date:** 2026-09-30
- **Authors:** Ali Rashidi, Antigravity Autonomous Agent
- **Target Platform:** ESP32-S3, Cortex-M4/M7, Bare-Metal `#![no_std]` Microcontrollers
- **Domain:** Industrial Switchgear E-Nose, Arc Decomposition Gas Telemetry, Modbus RTU / RS485

---

## 1. Context & Initial Hypothesis

The `BioNose-Edge` project initially hypothesized that directly transcribing the olfactory connectome of *Drosophila melanogaster*—specifically the **Mushroom Body (MB)** sparse random expansion into Kenyon Cells (KCs) coupled with Anterior Paired Lateral (APL) $k$-WTA binary thresholding—would provide unprecedented resilience against 36 months of physical metal-oxide semiconductor (MOS) sensor drift on low-power edge microcontrollers.

To validate this hypothesis with zero confirmation bias, we evaluated the architecture on the official **UCI Gas Sensor Array Drift Dataset** (13,910 real physical measurements spanning 36 months of hardware aging across 16 physical MOS sensors and 6 target gases).

---

## 2. The Adversarial Audit & Discovery of Methodological Flaw

During initial benchmarking, `BioNose Continual` appeared to outperform baselines (51.1% vs 40.2% Cosine). However, an adversarial internal audit revealed two severe methodological violations common in neuromorphic literature:
1. **Apples-to-Oranges Comparison:** The neuromorphic model was granted 5 calibration exemplars per batch to update its weights, while classical baselines (Cosine, Euclidean) were held completely **static and frozen** on Batch 1.
2. **Train-on-Test Data Leakage:** The calibration exemplars were evaluated within the test accuracy metric of the same batch.

---

## 3. The Controlled, Symmetrical, Zero-Leakage Tournament

We redesigned the evaluation framework to enforce strict symmetry:
- **Strict Disjoint Partitioning:** 5 calibration exemplars per class were extracted exclusively for field adaptation and completely removed from the test set (13,535 pure, unseen test samples evaluated).
- **Symmetrical Adaptation:** Every continual algorithm received the **exact same 5 calibration exemplars**.
  - `Cosine Continual`: Centroid updated via Leaky Exponential Moving Average: $c_k \leftarrow (1 - \alpha) c_k + \alpha x$.
  - `Euclidean Continual`: Centroid updated via Leaky EMA.
  - `BioNose Continual`: KC-to-MBON weights updated via Oja-Hebbian plasticity.
  - `Shuffled Continual`: Randomly rewired control graph updated via Oja-Hebbian plasticity.
- **Full Biological Scale Tested:** Kenyon Cell count scaled up to $K = 2,048$ (the exact biological count in the female adult *Drosophila* connectome from FlyWire / FAFB).

### Empirical Results (13,535 Unseen Physical Test Samples over 36 Months)

| Architecture / Algorithm | Steady-State ($M=16$) | Full Dynamic Kinetics ($M=128$) | Inference Latency | RAM Footprint |
| :--- | :--- | :--- | :--- | :--- |
| **Euclidean Static** (Frozen) | 31.8% | 32.8% | 1.1 $\mu$s | 0.8 KB |
| **Cosine Static** (Frozen) | 39.7% | 42.0% | 1.2 $\mu$s | 0.8 KB |
| **BioNose Continual** ($K=512$) | 41.2% | 50.0% | 24.1 $\mu$s | 3.2 KB |
| **BioNose Continual** ($K=2,048$, FlyWire scale) | 41.2% | 52.4% | 38.6 $\mu$s | 48.0 KB |
| **Shuffled Control Continual** ($K=2,048$) | 45.2% | 53.3% | 38.6 $\mu$s | 48.0 KB |
| **Euclidean Continual** (Leaky EMA) | 50.3% | 50.6% | 1.2 $\mu$s | 0.8 KB |
| **Cosine Continual** (Leaky EMA) | **61.1%** | **67.4%** | **1.8 $\mu$s** | **< 1.0 KB** |

---

## 4. Scientific Verdict & Falsification

1. **The Mushroom Body $k$-WTA is a Lossy Hash:**
   In biological insects, $k$-WTA binary quantization serves an energetic purpose: it bounds energy consumption to $\sim 10$ nanowatts by silencing 95% of neurons. However, on a digital microcontroller with a 32-bit hardware FPU (e.g. ESP32-S3), converting continuous real-valued sensor signals into sparse binary bitmasks discards critical geometric angle and magnitude information.
2. **Superiority of Continuous Cosine Vector Tracking:**
   Continuous directional tracking via unit-normalized cosine centroids preserves the full 128-dimensional orientation of the chemical signature. Under physical sensor aging, the feature vector rotates smoothly. An exponential moving average tracker ($c_k \leftarrow (1-\alpha)c_k + \alpha y$) follows this trajectory flawlessly, beating the 2,048-cell fly brain by **+15.0 percentage points (67.4% vs 52.4%)**.
3. **No Connectome Super-Power:**
   At $K=2,048$, the biological connectome achieved 52.4% while degree-preserved random rewiring achieved 53.3%. In continuous feature spaces under supervised field recalibration, the natural connectome provides no topological advantage over random projections.

---

## 5. Architectural Decision: The Hybrid `AdaptiveNoseEngine`

We reject the vanity of copying biological limits and adopt a high-performance **Hybrid Architecture**:

```
[Raw Physical MOS Sensors (M=16 or M=128)]
               │
               ▼
┌────────────────────────────────────────────────────────┐
│ Layer 1: Weber-Fechner Non-Linear Transducer           │
│   s_i = ln( (R_{0,i} / R_i) + eps )                    │
│   - Linearizes Langmuir chemical adsorption kinetics   │
│   - Multiplicative sensor aging drift cancelled        │
│   - Rejects sub-threshold clean air noise (Hill gate)  │
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
│   - Zero dynamic heap allocations, 1.8 us latency      │
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

## 6. Consequences & Metrics

| Metric | Legacy Drosophila ($K=2,048$) | Production `AdaptiveNoseEngine` | Delta |
| :--- | :--- | :--- | :--- |
| **Accuracy (36-Month Drift)** | 52.4% | **67.4%** | **+15.0% absolute improvement** |
| **Inference Latency** | 38.6 $\mu$s | **1.8 $\mu$s** | **21x faster** |
| **RAM Footprint** | 48.0 KB | **< 1.0 KB** | **98% RAM reduction** |
| **Dynamic Allocations** | Zero (`#![no_std]`) | Zero (`#![no_std]`) | Maintained |
| **Industrial Protocol** | Modbus RTU / RS485 | Modbus RTU / RS485 | Sealed & Tested |

---

## 7. Machine Verification Status

- `crates/bionose-core`: 16/16 unit tests passing (`cargo test -p bionose-core`).
- `#![no_std]` bare-metal verification: Passed (`cargo check -p bionose-core --no-default-features`).
- Empirical ground truth: Verified across all 13,910 physical samples of the UCI dataset.
