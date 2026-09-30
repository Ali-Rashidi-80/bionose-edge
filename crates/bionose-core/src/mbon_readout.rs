//! Mushroom Body Output Neurons (MBON) and Dopaminergic Plasticity Module.
//!
//! Implements associative readout with Oja's bounded Hebbian plasticity,
//! habituation to ambient background odors, and novelty detection.

use crate::mushroom_body::KcBitmask;

#[cfg(not(feature = "std"))]
#[inline]
fn math_sqrt(x: f32) -> f32 {
    libm::sqrtf(x)
}

#[cfg(feature = "std")]
#[inline]
fn math_sqrt(x: f32) -> f32 {
    x.sqrt()
}

/// Output prediction from the MBON readout layer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReadoutResult {
    /// Best matching class index (0..C-1).
    pub best_class: usize,
    /// Similarity score of the best matching class.
    pub similarity_score: f32,
    /// Indicates whether the odor signature is novel (below recognition threshold).
    pub is_novel: bool,
    /// Number of active Kenyon cells in this pattern.
    pub active_kc_count: u32,
}

/// Readout engine storing synaptic weights and habituation traces.
#[derive(Debug, Clone, Copy)]
pub struct MbonAssociator<const K: usize, const C: usize, const WORDS: usize> {
    /// Synaptic weight matrix [Class][Kenyon Cell].
    pub weights: [[f32; K]; C],
    /// Background ambient habituation trace per Kenyon Cell.
    pub habituation_trace: [f32; K],
    /// Habituation learning rate (lambda).
    pub habituation_rate: f32,
    /// Habituation suppression strength (mu).
    pub habituation_strength: f32,
    /// Minimum similarity score required to declare a recognized match (novelty boundary).
    pub novelty_threshold: f32,
    /// Track how many times each class has been updated.
    pub sample_counts: [u32; C],
}

impl<const K: usize, const C: usize, const WORDS: usize> MbonAssociator<K, C, WORDS> {
    /// Creates a new MBON associator initialized with zero weights and specified thresholds.
    pub const fn new(
        novelty_threshold: f32,
        habituation_rate: f32,
        habituation_strength: f32,
    ) -> Self {
        Self {
            weights: [[0.0f32; K]; C],
            habituation_trace: [0.0f32; K],
            habituation_rate,
            habituation_strength,
            novelty_threshold,
            sample_counts: [0u32; C],
        }
    }

    /// Evaluates the Kenyon cell bitmask against all learned classes.
    pub fn predict(&self, kc_mask: &KcBitmask<WORDS>) -> ReadoutResult {
        let mut best_class = 0;
        let mut best_score = -1.0f32;
        let mut active_count = 0u32;

        // Count total active Kenyon cells
        for w in 0..WORDS {
            active_count += kc_mask[w].count_ones();
        }

        if active_count == 0 {
            return ReadoutResult {
                best_class: 0,
                similarity_score: 0.0,
                is_novel: true,
                active_kc_count: 0,
            };
        }

        for c in 0..C {
            let mut score = 0.0f32;
            for w in 0..WORDS {
                let mut word = kc_mask[w];
                while word != 0 {
                    let bit_idx = word.trailing_zeros() as usize;
                    let k = w * 64 + bit_idx;
                    if k < K {
                        // Apply habituation suppression
                        let effective_act =
                            (1.0 - self.habituation_strength * self.habituation_trace[k]).max(0.0);
                        score += self.weights[c][k] * effective_act;
                    }
                    word &= word - 1; // Clear least significant bit
                }
            }

            if score > best_score {
                best_score = score;
                best_class = c;
            }
        }

        let is_novel = best_score < self.novelty_threshold || self.sample_counts[best_class] == 0;

        ReadoutResult {
            best_class,
            similarity_score: best_score,
            is_novel,
            active_kc_count: active_count,
        }
    }

    /// Trains the specified class with a single-shot or few-shot exposure using Oja's bounded rule.
    pub fn learn_association(&mut self, target_class: usize, kc_mask: &KcBitmask<WORDS>, eta: f32) {
        if target_class >= C {
            return;
        }

        // 1. Calculate current output response for target class
        let mut current_score = 0.0f32;
        for w in 0..WORDS {
            let mut word = kc_mask[w];
            while word != 0 {
                let bit_idx = word.trailing_zeros() as usize;
                let k = w * 64 + bit_idx;
                if k < K {
                    current_score += self.weights[target_class][k];
                }
                word &= word - 1;
            }
        }

        // 2. Oja's weight update: Delta W_k = eta * (a_k - y * W_k)
        for w in 0..WORDS {
            let mut word = kc_mask[w];
            while word != 0 {
                let bit_idx = word.trailing_zeros() as usize;
                let k = w * 64 + bit_idx;
                if k < K {
                    let a_k = 1.0f32;
                    let delta_w = eta * (a_k - current_score * self.weights[target_class][k]);
                    self.weights[target_class][k] =
                        (self.weights[target_class][k] + delta_w).max(0.0);
                }
                word &= word - 1;
            }
        }

        // 3. Normalize weight vector to unit norm to guarantee stability
        let mut sum_sq = 0.0f32;
        for k in 0..K {
            sum_sq += self.weights[target_class][k] * self.weights[target_class][k];
        }
        if sum_sq > 1e-6 {
            let norm = math_sqrt(sum_sq);
            for k in 0..K {
                self.weights[target_class][k] /= norm;
            }
        }

        self.sample_counts[target_class] += 1;
    }

    /// Updates the background habituation trace with the current sensory pattern.
    pub fn habituate_ambient(&mut self, kc_mask: &KcBitmask<WORDS>) {
        for w in 0..WORDS {
            let mut word = kc_mask[w];
            while word != 0 {
                let bit_idx = word.trailing_zeros() as usize;
                let k = w * 64 + bit_idx;
                if k < K {
                    self.habituation_trace[k] = (1.0 - self.habituation_rate)
                        * self.habituation_trace[k]
                        + self.habituation_rate * 1.0;
                }
                word &= word - 1;
            }
        }
    }

    /// Decays the habituation trace in clean air when no odors are present.
    pub fn decay_habituation(&mut self) {
        for k in 0..K {
            self.habituation_trace[k] *= 1.0 - self.habituation_rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_shot_learning_and_recall() {
        // K=128, C=4, WORDS=2
        let mut mbon = MbonAssociator::<128, 4, 2>::new(0.3, 0.05, 0.5);

        // Odor A: active bits at 10, 20, 30
        let mut odor_a = [0u64; 2];
        odor_a[0] = (1 << 10) | (1 << 20) | (1 << 30);

        // Initially novel
        let initial_res = mbon.predict(&odor_a);
        assert!(initial_res.is_novel);

        // Learn Odor A as Class 0
        mbon.learn_association(0, &odor_a, 0.2);

        // Recall Odor A
        let recall_res = mbon.predict(&odor_a);
        assert_eq!(recall_res.best_class, 0);
        assert!(!recall_res.is_novel);
        assert!(recall_res.similarity_score > 0.5);
    }

    #[test]
    fn test_habituation_suppression() {
        let mut mbon = MbonAssociator::<128, 4, 2>::new(0.2, 0.2, 0.8);
        let mut odor = [0u64; 2];
        odor[0] = (1 << 5) | (1 << 15);

        mbon.learn_association(1, &odor, 0.3);
        let score_before = mbon.predict(&odor).similarity_score;

        // Repeated ambient exposure triggers habituation
        for _ in 0..10 {
            mbon.habituate_ambient(&odor);
        }

        let score_after = mbon.predict(&odor).similarity_score;
        assert!(
            score_after < score_before,
            "Habituation must suppress familiar ambient odor: before={}, after={}",
            score_before,
            score_after
        );
    }
}
