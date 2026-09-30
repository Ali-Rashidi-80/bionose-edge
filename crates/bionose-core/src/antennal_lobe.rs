//! Antennal Lobe (AL) processing module with Divisive Normalization.
//!
//! Emulates local inhibitory interneurons (LNs) that pool global sensory energy
//! across all glomeruli to normalize projection neuron (PN) outputs:
//!
//! y_i = s_i / ( sigma + (1/M) * sum_{j=1}^M s_j )
//!
//! This grants concentration and baseline-drift invariance to chemical odor patterns.

/// Antennal Lobe circuit performing divisive normalization across M channels.
#[derive(Debug, Clone, Copy)]
pub struct AntennalLobe<const M: usize> {
    /// Semi-saturation constant (sigma). Prevents noise over-amplification in near-zero signals.
    pub sigma: f32,
}

impl<const M: usize> AntennalLobe<M> {
    /// Creates a new Antennal Lobe with a specified semi-saturation constant.
    pub const fn new(sigma: f32) -> Self {
        Self { sigma }
    }

    /// Default configuration for MOS chemical sensor arrays (sigma = 0.05).
    pub const fn default_config() -> Self {
        Self { sigma: 0.05 }
    }

    /// Performs divisive normalization on the input relative conductance vector.
    ///
    /// Computes the pooled inhibitory signal S = mean(s) and divides each channel
    /// by (sigma + S).
    pub fn normalize(&self, s: &[f32; M]) -> [f32; M] {
        let mut sum = 0.0f32;
        for i in 0..M {
            sum += s[i];
        }
        let mean = sum / (M as f32);
        let denom = self.sigma + mean;

        let mut y = [0.0f32; M];
        for i in 0..M {
            y[i] = s[i] / denom;
        }
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_invariance() {
        let al = AntennalLobe::<4>::new(0.01);
        let pattern_low = [1.0, 2.0, 4.0, 0.5];
        // Same chemical profile at 4x higher concentration
        let pattern_high = [4.0, 8.0, 16.0, 2.0];

        let y_low = al.normalize(&pattern_low);
        let y_high = al.normalize(&pattern_high);

        // High concentration should produce very similar relative ratio
        for i in 0..4 {
            let diff = (y_low[i] - y_high[i]).abs();
            assert!(
                diff < 0.15,
                "Divisive normalization should maintain scale invariance: diff at {} is {}",
                i,
                diff
            );
        }
    }
}
