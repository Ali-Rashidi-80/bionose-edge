//! Integrated BioNose Engine pipeline.
//!
//! Ties together Weber-Fechner transduction, Antennal Lobe divisive normalization,
//! Mushroom Body sparse projection, and MBON associative readout into a single
//! zero-allocation, embedded-ready engine.

use crate::antennal_lobe::AntennalLobe;
use crate::mbon_readout::{MbonAssociator, ReadoutResult};
use crate::mushroom_body::{KcBitmask, MushroomBody};
use crate::weber_fechner::WeberFechnerTransducer;

/// Configuration parameters for instantiating the BioNose pipeline.
#[derive(Debug, Clone, Copy)]
pub struct BioNoseConfig {
    pub default_r0: f32,
    pub epsilon: f32,
    pub orn_activation_threshold: f32,
    pub sigma: f32,
    pub lateral_inhibition_strength: f32,
    pub seed: u64,
    pub top_ratio: f32,
    pub noise_energy_threshold: f32,
    pub novelty_threshold: f32,
    pub habituation_rate: f32,
    pub habituation_strength: f32,
}

impl BioNoseConfig {
    /// Standard production defaults for industrial MOS arrays (e.g. BME688 / TGS sensors).
    pub const fn industrial_default() -> Self {
        Self {
            default_r0: 50_000.0,
            epsilon: 1e-6,
            orn_activation_threshold: 0.25,
            sigma: 0.05,
            lateral_inhibition_strength: 0.85,
            seed: 0x4452_4f53_4f50_4849, // "DROSOPHI"
            top_ratio: 0.05,             // 5% active Kenyon cells
            noise_energy_threshold: 0.08,
            novelty_threshold: 0.35,
            habituation_rate: 0.05,
            habituation_strength: 0.5,
        }
    }
}

/// Integrated Drosophila-inspired neuromorphic olfactory engine.
///
/// Const generics:
/// - M: Number of physical sensor channels (e.g. 10 for BME688, 16 for UCI dataset)
/// - K: Number of Kenyon cells (e.g. 128, 256, 512)
/// - D: Synaptic inputs per Kenyon cell (typically 4)
/// - C: Number of discrete chemical classes (e.g. 6 or 8)
/// - WORDS: Number of 64-bit words to hold K bits (WORDS = (K + 63) / 64)
#[derive(Debug, Clone, Copy)]
pub struct BioNoseEngine<
    const M: usize,
    const K: usize,
    const D: usize,
    const C: usize,
    const WORDS: usize,
> {
    pub transducer: WeberFechnerTransducer<M>,
    pub antennal_lobe: AntennalLobe<M>,
    pub mushroom_body: MushroomBody<M, K, D, WORDS>,
    pub mbon: MbonAssociator<K, C, WORDS>,
}

impl<
    const M: usize,
    const K: usize,
    const D: usize,
    const C: usize,
    const WORDS: usize,
> BioNoseEngine<M, K, D, C, WORDS> {
    /// Creates a new engine instance from configuration parameters.
    pub fn new(config: &BioNoseConfig) -> Self {
        Self {
            transducer: WeberFechnerTransducer::new(
                config.default_r0,
                config.epsilon,
                config.orn_activation_threshold,
            ),
            antennal_lobe: AntennalLobe::new(config.sigma, config.lateral_inhibition_strength),
            mushroom_body: MushroomBody::new(
                config.seed,
                config.top_ratio,
                config.noise_energy_threshold,
            ),
            mbon: MbonAssociator::new(
                config.novelty_threshold,
                config.habituation_rate,
                config.habituation_strength,
            ),
        }
    }

    /// Full forward inference pass from raw resistance values.
    ///
    /// Returns:
    /// - ReadoutResult: classification, similarity, novelty flag
    /// - KcBitmask: the 512-bit sparse binary representation of the odor
    pub fn infer(&self, raw_resistances: &[f32; M]) -> (ReadoutResult, KcBitmask<WORDS>) {
        // 1. Logarithmic transduction
        let s = self.transducer.transduce(raw_resistances);
        // 2. Divisive normalization
        let y = self.antennal_lobe.normalize(&s);
        // 3. Sparse projection & k-WTA
        let kc_mask = self.mushroom_body.forward(&y);
        // 4. MBON readout
        let result = self.mbon.predict(&kc_mask);

        (result, kc_mask)
    }

    /// Single-shot or few-shot associative training on a given target chemical class.
    pub fn train_sample(&mut self, raw_resistances: &[f32; M], target_class: usize, eta: f32) {
        let s = self.transducer.transduce(raw_resistances);
        let y = self.antennal_lobe.normalize(&s);
        let kc_mask = self.mushroom_body.forward(&y);
        self.mbon.learn_association(target_class, &kc_mask, eta);
    }

    /// Adapts the habituation filter to ongoing ambient background air.
    pub fn habituate_ambient(&mut self, raw_resistances: &[f32; M]) {
        let s = self.transducer.transduce(raw_resistances);
        let y = self.antennal_lobe.normalize(&s);
        let kc_mask = self.mushroom_body.forward(&y);
        self.mbon.habituate_ambient(&kc_mask);
    }

    /// Updates clean-air baseline resistances to track seasonal or thermal sensor drift.
    pub fn update_baseline(&mut self, clean_air_resistances: &[f32; M], alpha: f32) {
        self.transducer.update_baseline(clean_air_resistances, alpha);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_pipeline_flow() {
        // M=8, K=128, D=2, C=3, WORDS=2
        let config = BioNoseConfig {
            default_r0: 10_000.0,
            epsilon: 1e-6,
            orn_activation_threshold: 0.05,
            sigma: 0.05,
            lateral_inhibition_strength: 0.85,
            seed: 12345,
            top_ratio: 0.05,
            noise_energy_threshold: 0.05,
            novelty_threshold: 0.3,
            habituation_rate: 0.05,
            habituation_strength: 0.5,
        };

        let mut engine = BioNoseEngine::<8, 128, 2, 3, 2>::new(&config);

        // Odor 1: Sensor 0 strongly activated
        let odor_1 = [1_000.0, 9_500.0, 9_800.0, 10_000.0, 10_000.0, 10_000.0, 10_000.0, 10_000.0];
        // Odor 2: Sensor 7 strongly activated
        let odor_2 = [10_000.0, 10_000.0, 10_000.0, 10_000.0, 10_000.0, 9_800.0, 9_500.0, 1_000.0];

        // Before training, Odor 1 is novel
        let (res_pre, _) = engine.infer(&odor_1);
        assert!(res_pre.is_novel);

        // Train Odor 1 as Class 0, and Odor 2 as Class 1
        engine.train_sample(&odor_1, 0, 0.3);
        engine.train_sample(&odor_2, 1, 0.3);

        // Infer Odor 1: should be recognized as Class 0
        let (res_1, _) = engine.infer(&odor_1);
        assert_eq!(res_1.best_class, 0);
        assert!(!res_1.is_novel);

        // Infer Odor 2: should be recognized as Class 1
        let (res_2, _) = engine.infer(&odor_2);
        assert_eq!(res_2.best_class, 1);
        assert!(!res_2.is_novel);
    }
}
