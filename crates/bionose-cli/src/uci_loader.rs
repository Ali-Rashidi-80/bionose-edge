//! UCI Gas Sensor Array Drift Dataset parser and loader.
//!
//! Parses the official 10-batch dataset collected over 36 months by Alexander Vergara et al.
//! Format: SVMLight / LIBSVM: `<class_label> 1:<val1> 2:<val2> ... 128:<val128>`
//!
//! Labels:
//! 1: Ethanol
//! 2: Ethylene
//! 3: Ammonia
//! 4: Acetaldehyde
//! 5: Acetone
//! 6: Toluene

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct UciSample<const M: usize> {
    /// 0-indexed class (0 to 5)
    pub class_idx: usize,
    /// Sensory features
    pub features: [f32; M],
}

#[derive(Debug, Clone)]
pub struct UciBatch<const M: usize> {
    pub batch_id: usize,
    pub samples: Vec<UciSample<M>>,
}

/// Loads a single batch file.
/// If `steady_state_only` is true, extracts the 16 primary steady-state features (M=16).
/// If false, loads all 128 dynamic + steady-state features (M=128).
pub fn load_batch_16(path: &Path, batch_id: usize) -> Result<UciBatch<16>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut samples = Vec::new();

    // The 16 steady-state feature indices in 1-based indexing: 1, 9, 17, 25, 33, 41, 49, 57, 65, 73, 81, 89, 97, 105, 113, 121
    let ss_indices: [usize; 16] = [
        1, 9, 17, 25, 33, 41, 49, 57, 65, 73, 81, 89, 97, 105, 113, 121,
    ];

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut parts = trimmed.split_whitespace();
        let class_str = match parts.next() {
            Some(s) => s,
            None => continue,
        };

        let raw_class: usize = match class_str.parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        if raw_class == 0 || raw_class > 6 {
            continue;
        }
        let class_idx = raw_class - 1;

        let mut features = [0.0f32; 16];

        for token in parts {
            if let Some((idx_str, val_str)) = token.split_once(':') {
                if let (Ok(idx), Ok(val)) = (idx_str.parse::<usize>(), val_str.parse::<f32>()) {
                    for (sensor_num, &target_idx) in ss_indices.iter().enumerate() {
                        if idx == target_idx {
                            features[sensor_num] = val.abs(); // Resistance/voltage magnitude
                            break;
                        }
                    }
                }
            }
        }

        samples.push(UciSample {
            class_idx,
            features,
        });
    }

    Ok(UciBatch { batch_id, samples })
}

/// Loads all 10 batches from the dataset directory (16 steady-state features).
pub fn load_all_batches_16(dir: &Path) -> Result<Vec<UciBatch<16>>, std::io::Error> {
    let mut batches = Vec::new();
    for batch_id in 1..=10 {
        let filename = format!("batch{}.dat", batch_id);
        let path = dir.join(&filename);
        let batch = load_batch_16(&path, batch_id)?;
        batches.push(batch);
    }
    Ok(batches)
}

/// Loads a single batch with all 128 dynamic + steady-state features.
#[allow(dead_code)]
pub fn load_batch_128(path: &Path, batch_id: usize) -> Result<UciBatch<128>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut samples = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut parts = trimmed.split_whitespace();
        let class_str = match parts.next() {
            Some(s) => s,
            None => continue,
        };

        let raw_class: usize = match class_str.parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        if raw_class == 0 || raw_class > 6 {
            continue;
        }
        let class_idx = raw_class - 1;

        let mut features = [0.0f32; 128];

        for token in parts {
            if let Some((idx_str, val_str)) = token.split_once(':') {
                if let (Ok(idx), Ok(val)) = (idx_str.parse::<usize>(), val_str.parse::<f32>()) {
                    if (1..=128).contains(&idx) {
                        features[idx - 1] = val.abs();
                    }
                }
            }
        }

        samples.push(UciSample {
            class_idx,
            features,
        });
    }

    Ok(UciBatch { batch_id, samples })
}

/// Loads all 10 batches with all 128 features (M=128).
#[allow(dead_code)]
pub fn load_all_batches_128(dir: &Path) -> Result<Vec<UciBatch<128>>, std::io::Error> {
    let mut batches = Vec::new();
    for batch_id in 1..=10 {
        let filename = format!("batch{}.dat", batch_id);
        let path = dir.join(&filename);
        let batch = load_batch_128(&path, batch_id)?;
        batches.push(batch);
    }
    Ok(batches)
}
