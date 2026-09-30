//! BioNose-Core: Neuromorphic On-Device Olfactory Engine inspired by Drosophila melanogaster.
//!
//! Features:
//! - Weber-Fechner logarithmic transduction
//! - Antennal Lobe divisive normalization
//! - Mushroom Body sparse expansion with Energy-Gated k-WTA
//! - MBON associative readout with Oja's bounded plasticity and habituation
//! - Fully compatible with `#![no_std]` bare-metal microcontrollers (ESP32-S3, Cortex-M, RISC-V)
//! - Zero dynamic heap allocations (pure static memory)

#![cfg_attr(not(feature = "std"), no_std)]

pub mod antennal_lobe;
pub mod mbon_readout;
pub mod modbus;
pub mod mushroom_body;
pub mod pipeline;
pub mod weber_fechner;

// Re-exports for clean ergonomics
pub use antennal_lobe::AntennalLobe;
pub use mbon_readout::{MbonAssociator, ReadoutResult};
pub use modbus::{BioNoseTelemetry, ModbusException, ModbusSlave};
pub use mushroom_body::{
    bitmask_hamming, bitmask_overlap, KcBitmask, MushroomBody, ProjectionMatrix,
};
pub use pipeline::{BioNoseConfig, BioNoseEngine};
pub use weber_fechner::WeberFechnerTransducer;
