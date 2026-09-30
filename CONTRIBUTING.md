# Contributing to BioNose-Edge

We welcome technical contributions to `BioNose-Edge`. This project enforces strict zero-trust engineering standards, `#![no_std]` compliance, and empirical falsification.

---

## 1. Development Principles

1. **Deterministic Verification:** Every pull request must pass:
   - `cargo test --all` (all unit and adversarial stress tests green).
   - `cargo check -p bionose-core --no-default-features` (bare-metal `#![no_std]` compliance).
   - `cargo clippy --all` with **zero warnings**.
   - `cargo fmt --all -- --check`.
2. **Zero Dynamic Allocation (`#![no_std]`):** No usage of `alloc`, `std::vec::Vec`, `String`, or `Box` inside `bionose-core`. All state must be bounded in fixed static buffers or compile-time const generics.
3. **No 'Vibe-Coding':** Algorithmic claims must be backed by reproducible physical datasets (e.g. UCI Gas Sensor Array Drift Dataset).

---

## 2. Pull Request Workflow

1. Fork and create a feature branch (`git checkout -b feat/my-improvement`).
2. Implement your changes with accompanying unit tests.
3. Run the complete test suite:
   ```bash
   cargo test --all
   cargo clippy --all
   cargo fmt --all
   ```
4. Commit with Conventional Commits (`feat(core): ...`, `fix(modbus): ...`, `test(drift): ...`).
5. Open a Pull Request with concrete empirical verification metrics.
