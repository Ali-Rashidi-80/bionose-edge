# ADR-001: Drosophila Melanogaster Olfactory Neuromorphic Architecture

- **Status:** Superseded by [ADR-002](adr_002_drosophila_falsification_and_hybrid_pivot.md)
- **Date:** 2026-09-28
- **Author:** Ali Rashidi
- **Context:** Design of an edge olfactory engine for low-power gas sensor arrays.

---

## 1. Context and Problem Statement
Metal-Oxide Semiconductor (MOS) gas sensors suffer from non-linear adsorption kinetics, cross-sensitivity to environmental humidity, and severe baseline drift over multi-year deployments. Conventional deep learning architectures (MLP, 1D-CNN) deployed via TensorFlow Lite Micro require backpropagation on-device, which demands megabytes of SRAM and causes catastrophic forgetting.

Biological insects, specifically the fruit fly *Drosophila melanogaster*, solve real-time odor classification and odor plume tracking in chaotic turbulent air using approximately 2,000 Kenyon Cells with a power budget under 10 nanowatts.

## 2. Decision
We initially implemented the pure biological connectome of *Drosophila melanogaster* modeled after the FlyWire and Hemibrain connectome datasets:
1. **Olfactory Receptor Neurons (ORNs):** Logarithmic Weber-Fechner transduction linearizing Langmuir adsorption kinetics.
2. **Antennal Lobe (AL):** 54 Glomeruli with GABAergic local interneurons (LNs) performing subtractive lateral inhibition and divisive gain control.
3. **Mushroom Body (MB):** Sparse random projection into =2,048$ Kenyon Cells, followed by Winner-Take-All ($-WTA, top 5% firing rate) binary bitmask hashing.
4. **Mushroom Body Output Neurons (MBONs):** Linear readouts trained with Oja's biologically plausible Hebbian plasticity rule.

## 3. Consequences and Superseding
When benchmarked against the official 36-month UCI Gas Sensor Array Drift Dataset (13,910 physical samples), the pure biological Mushroom Body (=2,048$) achieved **52.4% accuracy**, falling short of continuous vector geometry (**67.4%**). 

The biological front-end (Layers 1 and 2) proved exceptionally effective at cancelling baseline drift and environmental humidity swings. However, binary $-WTA quantization (Layer 3) was falsified on digital FPUs.

This decision was formally superseded by **ADR-002 (Hybrid Pivot)**, which preserved biological Layers 1 & 2 while replacing Layer 3 with Continual Leaky Cosine Centroids.
