# Security Policy

## Supported Versions

| Version | Supported |
| :--- | :---: |
| `0.1.x` | :white_check_mark: |

---

## Reporting a Vulnerability

If you discover a security vulnerability, buffer overflow vector, or denial-of-service condition in `bionose-core` or its Modbus RTU protocol engine:

1. **Do not open a public issue.**
2. Email your technical disclosure to: `ali.rashidi@engineer.com` (or the repository maintainers).
3. Include:
   - Reproduction script or malformed frame payload.
   - Target architecture (e.g. ESP32-S3, Cortex-M4).
   - Traceback or fault analysis.

We will acknowledge receipt within 48 hours and coordinate a coordinated disclosure timeline.

---

## Memory Safety & Fuzzing Invariants

`bionose-core` is engineered under zero-trust assumptions:
- **Zero Unsafe:** The crate is 100% safe Rust.
- **Zero Heap:** No dynamic memory allocator is invoked; immune to heap overflow, use-after-free, or double-free exploits.
- **Buffer Bounding:** All incoming Modbus frames are strictly bounded to fixed array lengths, rejecting oversized payloads with Modbus Exception `0x03` or `0x04`.
