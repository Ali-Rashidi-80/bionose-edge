//! Mushroom Body (MB) sparse expansion module with Energy-Gated k-WTA.
//!
//! Projects M Projection Neurons into K Kenyon Cells via sparse pseudo-random
//! binary projections, followed by a Winner-Take-All (k-WTA) competition
//! regulated by Anterior Paired Lateral (APL) inhibitory feedback.
//!
//! Output is an ultra-sparse binary bitmask of active Kenyon cells.

/// Bitmask container for Kenyon Cell activations.
/// Each u64 holds 64 binary neuron states.
pub type KcBitmask<const WORDS: usize> = [u64; WORDS];

/// Computes the bitwise overlap (dot product / intersection size) between two Kenyon Cell bitmasks.
#[inline]
pub fn bitmask_overlap<const WORDS: usize>(a: &KcBitmask<WORDS>, b: &KcBitmask<WORDS>) -> u32 {
    let mut sum = 0u32;
    for i in 0..WORDS {
        sum += (a[i] & b[i]).count_ones();
    }
    sum
}

/// Computes the Hamming distance between two Kenyon Cell bitmasks.
#[inline]
pub fn bitmask_hamming<const WORDS: usize>(a: &KcBitmask<WORDS>, b: &KcBitmask<WORDS>) -> u32 {
    let mut sum = 0u32;
    for i in 0..WORDS {
        sum += (a[i] ^ b[i]).count_ones();
    }
    sum
}

/// Lightweight deterministic XorShift64 PRNG for static wiring generation.
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    const fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x8a5cd789635d2dff } else { seed },
        }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_bounded(&mut self, bound: usize) -> usize {
        (self.next() as usize) % bound
    }
}

/// Deterministic sparse projection matrix mapping M Projection Neurons to K Kenyon Cells.
/// Each Kenyon Cell samples exactly D Projection Neurons.
#[derive(Debug, Clone, Copy)]
pub struct ProjectionMatrix<const M: usize, const K: usize, const D: usize> {
    /// Connectivity table: for each Kenyon cell k, stores the D sampled PN indices.
    pub connections: [[usize; D]; K],
}

impl<const M: usize, const K: usize, const D: usize> ProjectionMatrix<M, K, D> {
    /// Generates a reproducible wiring topology using a deterministic seed.
    pub fn generate_deterministic(seed: u64) -> Self {
        let mut prng = XorShift64::new(seed);
        let mut connections = [[0usize; D]; K];

        for k in 0..K {
            let mut count = 0;
            while count < D {
                let candidate = prng.next_bounded(M);
                // Ensure distinct sampling for each Kenyon cell
                let mut already_chosen = false;
                for j in 0..count {
                    if connections[k][j] == candidate {
                        already_chosen = true;
                        break;
                    }
                }
                if !already_chosen {
                    connections[k][count] = candidate;
                    count += 1;
                }
            }
        }

        Self { connections }
    }

    /// Generates a shuffled control matrix with randomized rewiring for falsification testing.
    pub fn generate_shuffled_control(seed: u64) -> Self {
        Self::generate_deterministic(seed ^ 0xDEAD_BEEF_CAFE_BABE)
    }
}

/// Mushroom Body processing unit with energy gating and k-WTA.
#[derive(Debug, Clone, Copy)]
pub struct MushroomBody<const M: usize, const K: usize, const D: usize, const WORDS: usize> {
    pub projection: ProjectionMatrix<M, K, D>,
    /// Percentage of active Kenyon cells in the k-WTA competition (e.g. 0.05 for 5% top firing).
    pub top_ratio: f32,
    /// Minimum total sensory energy required to trigger Kenyon cell firing (noise gate).
    pub noise_energy_threshold: f32,
}

impl<const M: usize, const K: usize, const D: usize, const WORDS: usize>
    MushroomBody<M, K, D, WORDS>
{
    /// Creates a new Mushroom Body with deterministic wiring and specified sparsity ratio.
    pub fn new(seed: u64, top_ratio: f32, noise_energy_threshold: f32) -> Self {
        // Assert compile-time word size
        debug_assert!(
            WORDS >= K.div_ceil(64),
            "WORDS buffer must be sufficient to hold K bits"
        );
        Self {
            projection: ProjectionMatrix::generate_deterministic(seed),
            top_ratio,
            noise_energy_threshold,
        }
    }

    /// Computes the sparse binary bitmask of active Kenyon cells given normalized PN outputs.
    pub fn forward(&self, pn_outputs: &[f32; M]) -> KcBitmask<WORDS> {
        let mut bitmask = [0u64; WORDS];

        // 1. Calculate total sensory energy
        let mut total_energy = 0.0f32;
        for i in 0..M {
            total_energy += pn_outputs[i];
        }

        // 2. Energy gate: if below noise floor, silence all Kenyon cells
        if total_energy < self.noise_energy_threshold {
            return bitmask;
        }

        // 3. Linear projection to Kenyon cells
        let mut z = [0.0f32; K];
        for k in 0..K {
            let mut sum = 0.0f32;
            for j in 0..D {
                let pn_idx = self.projection.connections[k][j];
                sum += pn_outputs[pn_idx];
            }
            z[k] = sum;
        }

        // 4. Determine k-WTA threshold
        let target_k = ((K as f32) * self.top_ratio).max(1.0).min(K as f32) as usize;

        // Quick stack-based threshold discovery using partial selection
        let mut z_copy = z;
        let kth_threshold = quickselect_kth_largest(&mut z_copy, target_k);

        // 5. Populate binary bitmask for top firing neurons
        let mut active_count = 0;
        for k in 0..K {
            if z[k] >= kth_threshold && active_count < target_k {
                let word_idx = k / 64;
                let bit_idx = k % 64;
                bitmask[word_idx] |= 1u64 << bit_idx;
                active_count += 1;
            }
        }

        bitmask
    }
}

/// Helper function to find the kth largest element in a mutable slice using Quickselect.
fn quickselect_kth_largest(arr: &mut [f32], k: usize) -> f32 {
    let len = arr.len();
    if k == 0 || len == 0 {
        return 0.0;
    }
    let target_idx = len - k; // Index of the element if sorted ascending

    let mut left = 0;
    let mut right = len - 1;

    while left < right {
        let pivot_idx = partition(arr, left, right);
        if pivot_idx == target_idx {
            return arr[pivot_idx];
        } else if pivot_idx < target_idx {
            left = pivot_idx + 1;
        } else {
            if pivot_idx == 0 {
                break;
            }
            right = pivot_idx - 1;
        }
    }
    arr[left]
}

fn partition(arr: &mut [f32], left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut i = left;
    for j in left..right {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_wiring() {
        let p1 = ProjectionMatrix::<16, 128, 4>::generate_deterministic(42);
        let p2 = ProjectionMatrix::<16, 128, 4>::generate_deterministic(42);
        for k in 0..128 {
            assert_eq!(p1.connections[k], p2.connections[k]);
        }
    }

    #[test]
    fn test_energy_gating() {
        // K=128 -> WORDS = 2
        let mb = MushroomBody::<8, 128, 4, 2>::new(100, 0.05, 1.0);
        let weak_input = [0.05; 8]; // Total energy = 0.4 < 1.0
        let mask = mb.forward(&weak_input);
        assert_eq!(mask[0], 0);
        assert_eq!(mask[1], 0);
    }

    #[test]
    fn test_sparsity_enforcement() {
        // K=128, top_ratio=0.05 -> target_k = 6 active bits
        let mb = MushroomBody::<8, 128, 4, 2>::new(100, 0.05, 0.1);
        let active_input = [2.0, 1.5, 0.2, 0.8, 3.0, 0.0, 1.1, 0.5];
        let mask = mb.forward(&active_input);
        let total_active = mask[0].count_ones() + mask[1].count_ones();
        assert_eq!(
            total_active, 6,
            "Expected exactly 6 active Kenyon cells for 5% of 128"
        );
    }
}
