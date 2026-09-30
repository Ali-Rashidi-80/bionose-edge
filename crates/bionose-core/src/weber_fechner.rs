//! Weber-Fechner non-linear transduction module.
//!
//! Maps raw metal-oxide semiconductor (MOS) sensor resistances to logarithmic
//! relative conductance, linearizing the sub-linear Langmuir-Hinshelwood adsorption kinetics:
//!
//! s_i = ln( (R_{0,i} / R_i) + epsilon )

#[cfg(not(feature = "std"))]
#[inline]
fn math_ln(x: f32) -> f32 {
    libm::logf(x)
}

#[cfg(feature = "std")]
#[inline]
fn math_ln(x: f32) -> f32 {
    x.ln()
}

/// Transducer that converts raw resistance vectors into logarithmic relative conductance.
#[derive(Debug, Clone, Copy)]
pub struct WeberFechnerTransducer<const M: usize> {
    /// Baseline clean-air resistance per sensor channel in Ohms.
    pub r0: [f32; M],
    /// Numerical regularization constant to prevent log(0) or division by zero.
    pub epsilon: f32,
    /// Minimum allowed physical resistance reading to prevent sensor short-circuit artifacts.
    pub r_min: f32,
    /// ORN activation threshold (ln ratio): rejects sub-threshold ambient weather/humidity fluctuations.
    pub activation_threshold: f32,
}

impl<const M: usize> WeberFechnerTransducer<M> {
    /// Creates a new transducer with initial clean-air baselines and activation threshold.
    pub const fn new(default_r0: f32, epsilon: f32, activation_threshold: f32) -> Self {
        Self {
            r0: [default_r0; M],
            epsilon,
            r_min: 1.0, // 1 Ohm floor
            activation_threshold,
        }
    }

    /// Creates a new transducer with channel-specific baseline resistances.
    pub const fn with_baselines(r0: [f32; M], epsilon: f32, activation_threshold: f32) -> Self {
        Self {
            r0,
            epsilon,
            r_min: 1.0,
            activation_threshold,
        }
    }

    /// Calibrates the baseline clean-air resistances using an exponential moving average.
    ///
    /// alpha: adaptation rate (e.g. 0.05 for slow baseline tracking).
    pub fn update_baseline(&mut self, fresh_air_resistances: &[f32; M], alpha: f32) {
        let alpha = alpha.clamp(0.0, 1.0);

        for i in 0..M {
            let r = fresh_air_resistances[i].max(self.r_min);
            self.r0[i] = (1.0 - alpha) * self.r0[i] + alpha * r;
        }
    }

    /// Transforms raw sensor resistances into logarithmic relative conductance.
    ///
    /// Sub-threshold weather fluctuations (ln(R0/R) < activation_threshold) are zeroed out,
    /// mimicking insect olfactory receptor neuron (ORN) chemical binding affinity thresholds.
    pub fn transduce(&self, raw_resistances: &[f32; M]) -> [f32; M] {
        let mut s = [0.0f32; M];
        for i in 0..M {
            let r = raw_resistances[i].max(self.r_min);
            let r0 = self.r0[i].max(self.r_min);
            let ratio = (r0 / r) + self.epsilon;
            let val = math_ln(ratio);
            let gated = val - self.activation_threshold;
            s[i] = if gated < 0.0 { 0.0 } else { gated };
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_air_near_zero() {
        let transducer = WeberFechnerTransducer::<4>::new(10_000.0, 1e-6, 0.0);
        let clean_air = [10_000.0; 4];
        let s = transducer.transduce(&clean_air);
        for &val in &s {
            assert!(
                val >= 0.0 && val < 0.01,
                "Clean air should yield near-zero response"
            );
        }
    }

    #[test]
    fn test_reducing_gas_positive_conductance() {
        let transducer = WeberFechnerTransducer::<4>::new(10_000.0, 1e-6, 0.0);
        // Resistance drops to 2,000 Ohms (gas presence)
        let gas_exposure = [2_000.0, 5_000.0, 10_000.0, 1_000.0];
        let s = transducer.transduce(&gas_exposure);

        assert!(s[0] > s[1]);
        assert!(s[1] > s[2]);
        assert!(s[3] > s[0]);
    }
}
