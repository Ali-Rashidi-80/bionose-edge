//! Antennal Lobe (AL) processing module with Divisive Normalization.
//!
//! Emulates local inhibitory interneurons (LNs) that pool global sensory energy
//! across all glomeruli to normalize projection neuron (PN) outputs:
//!
//! y_i = s_i / ( sigma + (1/M) * sum_{j=1}^M s_j )
//!
//! This grants concentration and baseline-drift invariance to chemical odor patterns.

/// Antennal Lobe circuit performing subtractive lateral inhibition and divisive normalization.
#[derive(Debug, Clone, Copy)]
pub struct AntennalLobe<const M: usize> {
    /// Semi-saturation constant (sigma). Prevents noise over-amplification in near-zero signals.
    pub sigma: f32,
    /// Subtractive lateral inhibition strength (beta) to cancel common-mode weather/humidity shifts.
    pub lateral_inhibition_strength: f32,
}

impl<const M: usize> AntennalLobe<M> {
    /// Creates a new Antennal Lobe with specified sigma and lateral inhibition strength.
    pub const fn new(sigma: f32, lateral_inhibition_strength: f32) -> Self {
        Self {
            sigma,
            lateral_inhibition_strength,
        }
    }

    /// Default configuration for MOS chemical sensor arrays (sigma = 0.05, beta = 0.85).
    pub const fn default_config() -> Self {
        Self {
            sigma: 0.05,
            lateral_inhibition_strength: 0.85,
        }
    }

    /// Performs lateral inhibition (common-mode rejection) followed by divisive normalization.
    pub fn normalize(&self, s: &[f32; M]) -> [f32; M] {
        // 1. Calculate population mean (common-mode background)
        let mut sum = 0.0f32;
        for i in 0..M {
            sum += s[i];
        }
        let mean = sum / (M as f32);

        // 2. Subtractive lateral inhibition: subtract shared common-mode activity
        let mut s_diff = [0.0f32; M];
        let beta = self.lateral_inhibition_strength;
        for i in 0..M {
            let val = (s[i] - beta * mean).max(0.0);
            s_diff[i] = val;
        }

        // 3. Divisive normalization against total sensory drive (sigma + mean(s))
        // This ensures common-mode shifts shrink to zero instead of being rescaled back up.
        let denom = self.sigma + mean;

        let mut y = [0.0f32; M];
        for i in 0..M {
            y[i] = s_diff[i] / denom;
        }
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_invariance() {
        let al = AntennalLobe::<4>::new(0.01, 0.5);
        let pattern_low = [1.0, 2.0, 4.0, 0.5];
        // Same chemical profile at 4x higher concentration
        let pattern_high = [4.0, 8.0, 16.0, 2.0];

        let y_low = al.normalize(&pattern_low);
        let y_high = al.normalize(&pattern_high);

        // High concentration should produce very similar relative ratio
        for i in 0..4 {
            let diff = (y_low[i] - y_high[i]).abs();
            assert!(
                diff < 0.25,
                "Divisive normalization should maintain scale invariance: diff at {} is {}",
                i,
                diff
            );
        }
    }

    #[test]
    fn test_common_mode_weather_rejection() {
        // When all sensors drop uniformly (clean air humidity change)
        let al = AntennalLobe::<4>::new(0.05, 0.95);
        let humidity_swing = [0.8, 0.8, 0.8, 0.8]; // Pure common-mode
        let y = al.normalize(&humidity_swing);

        let mut energy = 0.0f32;
        for &val in &y {
            energy += val;
        }
        assert!(
            energy < 0.25,
            "Lateral inhibition must suppress common-mode weather shifts (from 3.76 down to <0.25): energy is {}",
            energy
        );
    }
}
