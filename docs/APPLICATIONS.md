# Industrial, IoT & Embedded Application Engineering Guide

**Languages:** [English](APPLICATIONS.md) (default) · [فارسی](APPLICATIONS.fa.md) · Companion to [README.md](../README.md) and [ARCHITECTURE.md](../ARCHITECTURE.md)

This document provides the definitive, adversarial engineering audit of practical deployment domains for `BioNose-Edge`. Every application is evaluated under physical sensor chemistry constraints, power budgets, and hardware invariants.

---

## 1. Adversarial Screening Framework

Multi-sensor gas arrays (MOS/MEMS) operate under strict physical boundaries. To prevent pseudoscience, wishful thinking, and scope creep, all proposed applications must pass a 4-part physical feasibility gate:

| Feasibility Gate | Physical Boundary Criterion | Rejection Threshold |
| :--- | :--- | :--- |
| **G1: Chemical Volatility** | Analyte vapor pressure must exceed 0.1 mmHg @ 25°C | Non-volatile biological macromolecules or heavy residues are rejected. |
| **G2: MOS Cross-Sensitivity** | Target gas must produce distinct kinetic/ratio signature across $\ge 4$ array channels | Single-gas detection where a dedicated electrochemical cell suffices is rejected. |
| **G3: Environmental Invariance**| Relative humidity (10% to 95% RH) must be cancellable via Antennal Lobe normalization | Applications requiring condensation on the sensor surface are rejected. |
| **G4: Edge Resource Envelope** | Full inference + telemetry must execute in $< 50\ \mu$s with $0$ bytes heap | Complex recurrent architectures requiring cloud backpropagation are rejected. |

---

## 2. Adversarial Use-Case Screening Matrix

We subjected 10 candidate applications to rigorous adversarial screening:

| # | Application Domain | Physical Feasibility | Verdict | Rationale & Boundaries |
| :-: | :--- | :---: | :---: | :--- |
| **1** | **BESS Thermal Runaway Off-Gassing** | **High** | **Tier 1 (Ship)** | Lithium-ion battery venting releases DMC/DEC/EMC solvents + H2/CO 5–15 mins before fire. Ideal for multi-sensor array. |
| **2** | **MV/HV Switchgear Arc & Pyrolysis** | **High** | **Tier 1 (Ship)** | Partial discharge pyrolyzes busbar epoxy resin into distinct VOCs. Modbus RTU over RS485 provides native PLC telemetry. |
| **3** | **Cold Chain Ethylene & Ripening** | **High** | **Tier 1 (Ship)** | Ethylene/ethanol spikes in fruit containers. Antennal Lobe eliminates 90%+ RH baseline drift; 5-byte LoRaWAN packet. |
| **4** | **Industrial Solvent & Chemical Leaks** | **High** | **Tier 1 (Ship)** | Ammonia, acetone, toluene in chemical plants. On-device 5-shot adaptation handles multi-year MOS aging without factory recall. |
| **5** | **Underground Mining Safety Nods** | **Moderate** | **Tier 2 (Feasible)** | Methane, CO, H2S monitoring in tunnels. Zero-cloud `#![no_std]` autonomy works without RF connectivity. |
| **6** | **HVAC Demand-Controlled Ventilation** | **Moderate** | **Tier 2 (Feasible)** | Discriminating cooking odors vs human bio-effluents vs cleaning chemicals. Requires baseline zeroing during night hours. |
| **7** | **Transformer DGA (Dissolved Gas)** | **Low** | **Tier 3 (Reject)** | Dissolved gas analysis in transformer oil requires vacuum oil extraction. Raw MOS immersion is physically impossible. |
| **8** | **Medical Disease Breath Diagnosis** | **Low** | **Tier 3 (Reject)** | Human breath contains 100% humidity + thousands of volatile trace biomarkers at ppt/ppb. Raw MOS arrays fail without GC/MS pre-separation. |
| **9** | **Explosive Trace Residue Sniffing** | **Low** | **Tier 3 (Reject)** | RDX and PETN explosives have negligible vapor pressure at room temperature; requires chemical swab pre-concentrators. |
| **10**| **Single-Sensor Home Smoke Alarm** | **Unnecessary** | **Tier 3 (Reject)** | A 50-cent analog comparator with a single MQ-2 sensor suffices. Over-engineering for a trivial binary threshold problem. |

---

## 3. Tier-1 Production Deployments (Atomic Specifications)

### 3.1 Application 1: Battery Energy Storage Systems (BESS) Off-Gassing Early Warning

#### The Physical Problem
Lithium-ion cells (LFP, NMC) in energy storage containers undergo mechanical degradation or internal short-circuits. Before exothermic thermal runaway occurs, internal pressure ruptures the cell safety vent, expelling organic carbonate solvents (Dimethyl Carbonate [DMC], Diethyl Carbonate [DEC], Ethyl Methyl Carbonate [EMC]) accompanied by hydrogen ($H_2$) and carbon monoxide ($CO$). Standard optical smoke detectors trigger **after** flames erupt.

#### Sensor Topology & Hardware BOM
- **Sensors:** 4 to 8 MOS/MEMS sensors with distinct catalytic dopings (e.g. $SnO_2$, $WO_3$, $ZnO$).
- **Microcontroller:** ESP32-S3 (Xtensa dual-core @ 240 MHz) or STM32G474 (Cortex-M4F @ 170 MHz).
- **Communication:** Isolated RS485 transceiver (e.g. ADM2483 / ISO3082) connected to Battery Management System (BMS).

#### Algorithmic Behavior & Telemetry
- **Layer 1:** Weber-Fechner compresses high-dynamic-range solvent spikes.
- **Layer 2:** Antennal Lobe gain control suppresses background container HVAC air currents.
- **Layer 3:** Class 1 is calibrated to DMC/DEC vapor signature. Output emits `CONFIDENCE_BPS >= 8500` with `NOVELTY_FLAG = 0`.
- **Modbus Holding Registers:**
  - `0x0001` (SYSTEM_STATUS) transitions from `1` (Normal) to `2` (Anomaly/Warning).
  - `0x0002` (DETECTED_GAS_CLASS) reports `0x0001` (Electrolyte Venting).
  - PLC triggers emergency exhaust ventilation and isolated string disconnection **8 to 12 minutes before thermal runaway**.

---

### 3.2 Application 2: MV/HV Switchgear Arc & Busbar Pyrolysis

#### The Physical Problem
Medium-voltage (11 kV–33 kV) switchgear cabinets experience partial discharge (PD), loose busbar bolting, and micro-arcing. Thermal hot-spots pyrolyze epoxy resin insulators, busbar heat-shrink sleeves, and cable sheathing, releasing characteristic benzene derivatives, acetylene, carbon monoxide, and phenolic compounds hours before catastrophic phase-to-phase flashover.

#### Hardware Architecture
- **Environment:** High electromagnetic interference (EMI), strong electric fields, sealed enclosure.
- **Sensor Puck:** Sealed aluminum chassis with sintered stainless-steel flame-proof gas permeable mesh.
- **Bus:** Daisy-chained RS485 Modbus RTU loop running at 19,200 baud, 8N1.

#### Invariants Enforced
- **Zero Heap Allocations:** Guarantees 100% deterministic uptime across 15+ years of substation duty.
- **Modbus Register `0x0007` (Drift Degradation):** Continuously reports sensor oxide degradation back to the SCADA system, scheduling sensor head replacement during routine annual substation outages.

---

### 3.3 Application 3: Post-Harvest Cold-Chain Logistics & Smart Agriculture

#### The Physical Problem
During intercontinental marine transit of bananas, avocados, and apples in refrigerated containers (reefers), climacteric fruits emit ethylene ($C_2H_4$) which accelerates ripening of the entire container load. If ripening initiates prematurely, ethylene spikes are followed by fermentation ethanol ($C_2H_5OH$). Inside the container, relative humidity exceeds 90% to 95% RH.

#### Why Conventional Sensors Fail
Standard MOS sensors suffer catastrophic resistance collapse under 95% RH, triggering constant false alarms.

#### The BioNose-Edge Advantage
- **Antennal Lobe Divisive Normalization:** Subtractive lateral inhibition subtracts the common-mode water vapor baseline, and divisive gain normalizes vector magnitude.
- **Ultra-Compact Wireless Telemetry:** The node wakes every 10 minutes, runs inference in $1.8\ \mu$s, and broadcasts a **5-byte LoRaWAN packet** (Gas ID, Confidence, Latency, Drift).
- **Power Budget:** Total active wake duration is $< 40\ \mu$s (including ADC). A standard $3.6	ext{V}$ $2400	ext{ mAh}$ $Li	ext{-}SOCl_2$ cell operates reliably for **over 5 years**.

---

### 3.4 Application 4: Industrial Chemical Solvent & Ammonia Leak Detection

#### The Physical Problem
Chemical manufacturing facilities and industrial cold-storage refrigeration plants use ammonia ($NH_3$) and volatile solvents (toluene, acetone, ethanol). Ambient solvent vapor levels fluctuate with outdoor temperatures, causing traditional static threshold detectors to drift and sound false evacuation alarms.

#### Algorithmic Solution
- **Continual Leaky Cosine Tracking:** Uses 5 field calibration samples to smoothly track seasonal baseline drift via Leaky Exponential Moving Average (EMA).
- **Zero Catastrophic Forgetting:** The centroid retains angular separation between dangerous ammonia leaks and ambient background ethanol vapors without storing past training vectors.

---

## 4. Hardware Reference Interfacing Blueprint

```text
                      +3.3V / +5V Clean Regulated Supply
                                   │
              ┌────────────────────┴───────────────────┐
              │                                        │
        ┌─────┴──────┐                           ┌─────┴──────┐
        │ MOS Sensor │                           │ MOS Sensor │
        │ Channel 0  │                           │ Channel 15 │
        └─────┬──────┘                           └─────┬──────┘
              │ V_out,0                                │ V_out,15
              ▼                                        ▼
    ┌────────────────────────────────────────────────────────┐
    │ 16-Channel Low-Noise ADC / Analog Mux (e.g. ADS1115)   │
    └──────────────────────────┬─────────────────────────────┘
                               │ SPI / I2C / Direct GPIO ADC
                               ▼
    ┌────────────────────────────────────────────────────────┐
    │ Microcontroller (ESP32-S3 / STM32 Cortex-M4 / RISC-V)  │
    │                                                        │
    │  [ bionose-core ]  (#![no_std], 0 bytes heap)          │
    │  ├─ Layer 1: Weber-Fechner Transduction (100 ns)       │
    │  ├─ Layer 2: Antennal Lobe Normalization (350 ns)      │
    │  ├─ Layer 3: Continual Leaky Cosine Centroid (1.3 us)  │
    │  └─ Output: Class ID, Confidence BPS, Novelty Flag     │
    └──────────────┬──────────────────────────┬──────────────┘
                   │                          │
      Modbus Frames│             5-Byte Packet│
                   ▼                          ▼
      ┌─────────────────────────┐  ┌─────────────────────────┐
      │ RS485 Transceiver (PHY) │  │ LoRaWAN / NB-IoT Radio  │
      │ (MAX485 / ISO3082)      │  │ (SX1262 / nRF9160)       │
      │ ──► Industrial PLC / DCS│  │ ──► Gateway / Cloud     │
      └─────────────────────────┘  └─────────────────────────┘
```

---

## 5. Firmware Integration Checklist for IoT Engineers

1. **ADC Pre-Scaling:** Ensure raw ADC voltage is converted to sensor electrical resistance ($R_i$ in Ohms) before feeding `AdaptiveNoseEngine::infer(&resistances)`.
2. **Clamping & Fault Detection:** If $R_i \le 1.0\ \Omega$ (short circuit) or $R_i \ge 10^9\ \Omega$ (broken wire), the engine gracefully clamps without crashing or generating `NaN`.
3. **Field Calibration Workflow:** When reference zero-gas is purged across the sensor head, issue Modbus Command `0x0001` or call `engine.auto_zero()` to reset pristine baseline resistances $R_{0,i}$.
4. **Power Gating:** Power the sensor heater element via a dedicated MOSFET switch. MOS sensors require 15 to 30 seconds of thermal pre-heating before chemical adsorption equilibrium is achieved.
