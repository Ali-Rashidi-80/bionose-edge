# Changelog

All notable changes to `BioNose-Edge` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-09-30

### Added
- **Production `AdaptiveNoseEngine`:** High-precision hybrid engine integrating Weber-Fechner non-linear transduction, Antennal Lobe divisive normalization, and continuous-space Leaky Cosine Centroid tracking.
- **Bare-Metal Modbus RTU / RS485 Protocol Slave:** Zero-allocation frame parser and response generator supporting Function Codes `0x03`, `0x06`, and `0x10`.
- **Adversarial Fault-Injection Suite:** 8 integration stress tests verifying sensor short circuits (0.0 $\Omega$), open circuits ($10^{10}\ \Omega$), Modbus CRC16 corruption, and buffer overflow rejection.
- **Authentic Physical Benchmark Harness:** 10-batch UCI Gas Sensor Array Drift Dataset loader evaluating 13,910 physical measurements across 36 months of sensor aging.
- **Architecture Decision Records:** ADR-001 (Drosophila baseline architecture) and ADR-002 (empirical falsification of Mushroom Body $k$-WTA binary expansion and pivot to continuous cosine tracking).
- **Dual Licensing:** Apache-2.0 and MIT.
- **Bilingual Documentation:** Comprehensive English (`README.md`) and Persian (`README.fa.md`) project specifications with 3D isometric assets.
