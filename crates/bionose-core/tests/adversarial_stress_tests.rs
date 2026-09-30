//! Adversarial Stress Testing & Fault-Injection Suite for BioNose-Core.
//!
//! Validates:
//! 1. Extreme physical sensor faults (Short circuits, Open circuits, High-voltage spikes).
//! 2. Mathematical invariant safety (No NaNs, No Infinities, No Panics, Division-by-Zero immunity).
//! 3. Industrial Modbus RTU adversarial protocol fuzzing (CRC corruption, Overflow protection, Exception boundaries).
//! 4. Continual adaptation numerical stability under 10,000 rapid drift cycles.

use bionose_core::{
    AdaptiveNoseConfig, AdaptiveNoseEngine, BioNoseTelemetry, ModbusException, ModbusSlave,
};

#[test]
fn test_sensor_fault_injection_short_circuit_immunity() {
    let config = AdaptiveNoseConfig::industrial_default();
    let engine = AdaptiveNoseEngine::<4, 2>::new(&config);

    // Sensor 0 is physically dead shorted to Ground (0.000 Ohms)
    // Sensor 1 is sub-Ohm bridge (0.01 Ohms)
    let dead_short = [0.0, 0.01, 45_000.0, 50_000.0];

    // Must execute cleanly without panic, NaN, or Inf
    let result = engine.infer(&dead_short);
    assert!(!result.similarity.is_nan(), "Similarity must not be NaN");
    assert!(
        !result.similarity.is_infinite(),
        "Similarity must not be Infinite"
    );
    assert!(
        !result.signal_magnitude.is_nan(),
        "Magnitude must not be NaN"
    );
}

#[test]
fn test_sensor_fault_injection_open_circuit_infinity() {
    let config = AdaptiveNoseConfig::industrial_default();
    let engine = AdaptiveNoseEngine::<4, 2>::new(&config);

    // Sensor 0 is physically disconnected / open circuit (10 GigaOhms)
    let open_circuit = [10_000_000_000.0, 50_000.0, 48_000.0, 49_000.0];

    let result = engine.infer(&open_circuit);
    assert!(!result.similarity.is_nan());
    assert!(!result.similarity.is_infinite());
    assert!(!result.signal_magnitude.is_nan());
}

#[test]
fn test_modbus_adversarial_crc_corruption_silent_drop() {
    let slave = ModbusSlave::new(1);
    let mut telemetry = BioNoseTelemetry::default();
    let mut tx_buf = [0u8; 64];

    // Valid query: Read 2 registers starting at 0x0001
    // [0x01, 0x03, 0x00, 0x01, 0x00, 0x02, CRC_lo, CRC_hi]
    let mut req = [0x01, 0x03, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00];
    let crc = bionose_core::modbus::calculate_crc16(&req[..6]);
    req[6] = (crc & 0xFF) as u8;
    req[7] = (crc >> 8) as u8;

    // 1. Valid frame must succeed
    let res = slave.process_frame(&req, &mut telemetry, &mut tx_buf);
    assert!(res.is_some());

    // 2. Corrupt 1 bit in CRC
    req[6] ^= 0x01;
    let corrupted_res = slave.process_frame(&req, &mut telemetry, &mut tx_buf);
    assert_eq!(
        corrupted_res, None,
        "Corrupted CRC frame must be dropped silently per Modbus spec"
    );

    // 3. Truncated frame (less than 4 bytes)
    let truncated = [0x01, 0x03];
    assert_eq!(
        slave.process_frame(&truncated, &mut telemetry, &mut tx_buf),
        None
    );
}

#[test]
fn test_modbus_adversarial_buffer_overflow_defense() {
    let slave = ModbusSlave::new(1);
    let mut telemetry = BioNoseTelemetry::default();

    // Adversarial Query: Request 100 registers (reg_count = 100 = 0x0064)
    // Frame: [0x01, 0x03, 0x00, 0x01, 0x00, 0x64, CRC_lo, CRC_hi]
    let mut req = [0x01, 0x03, 0x00, 0x01, 0x00, 0x64, 0x00, 0x00];
    let crc = bionose_core::modbus::calculate_crc16(&req[..6]);
    req[6] = (crc & 0xFF) as u8;
    req[7] = (crc >> 8) as u8;

    let mut small_tx_buf = [0u8; 16]; // Deliberately too small for 200 bytes of response
    let res = slave.process_frame(&req, &mut telemetry, &mut small_tx_buf);

    assert!(res.is_some());
    let len = res.unwrap();
    // Must return an exception response (5 bytes), NOT overflow memory or panic
    assert_eq!(len, 5);
    assert_eq!(small_tx_buf[1], 0x83); // Exception on 0x03
    assert_eq!(small_tx_buf[2], ModbusException::IllegalDataValue as u8);
}

#[test]
fn test_modbus_adversarial_illegal_function_code() {
    let slave = ModbusSlave::new(1);
    let mut telemetry = BioNoseTelemetry::default();
    let mut tx_buf = [0u8; 64];

    // Adversarial function code 0x17 (Read/Write Multiple) not supported
    let mut req = [0x01, 0x17, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00];
    let crc = bionose_core::modbus::calculate_crc16(&req[..6]);
    req[6] = (crc & 0xFF) as u8;
    req[7] = (crc >> 8) as u8;

    let len = slave
        .process_frame(&req, &mut telemetry, &mut tx_buf)
        .expect("Handled");
    assert_eq!(len, 5);
    assert_eq!(tx_buf[1], 0x17 | 0x80); // 0x97
    assert_eq!(tx_buf[2], ModbusException::IllegalFunction as u8);
}

#[test]
fn test_modbus_adversarial_illegal_address() {
    let slave = ModbusSlave::new(1);
    let mut telemetry = BioNoseTelemetry::default();
    let mut tx_buf = [0u8; 64];

    // Query non-existent register 0x0099
    let mut req = [0x01, 0x03, 0x00, 0x99, 0x00, 0x01, 0x00, 0x00];
    let crc = bionose_core::modbus::calculate_crc16(&req[..6]);
    req[6] = (crc & 0xFF) as u8;
    req[7] = (crc >> 8) as u8;

    let len = slave
        .process_frame(&req, &mut telemetry, &mut tx_buf)
        .expect("Handled");
    assert_eq!(len, 5);
    assert_eq!(tx_buf[1], 0x83);
    assert_eq!(tx_buf[2], ModbusException::IllegalDataAddress as u8);
}

#[test]
fn test_continual_adaptation_long_term_numerical_stability() {
    let config = AdaptiveNoseConfig::industrial_default();
    let mut engine = AdaptiveNoseEngine::<4, 2>::new(&config);

    // Initial training
    let base_odor = [2_000.0, 48_000.0, 49_000.0, 50_000.0];
    engine.train_sample(&base_odor, 0);

    // Perform 10,000 rapid adaptation cycles with simulated drift fluctuations
    let mut state = 123456789u64;
    for _ in 0..10_000 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let delta = ((state % 1000) as f32) - 500.0;

        let drifted_reading = [
            (2_000.0 + delta).max(10.0),
            (48_000.0 + delta).max(100.0),
            (49_000.0 + delta).max(100.0),
            (50_000.0 + delta).max(100.0),
        ];

        engine.adapt_field_sample(&drifted_reading, 0, 0.05);
    }

    // Verify centroids never diverged, overflowed, or produced NaN
    for i in 0..4 {
        assert!(
            !engine.centroids[0][i].is_nan(),
            "Centroid must remain valid"
        );
        assert!(
            !engine.centroids[0][i].is_infinite(),
            "Centroid must remain finite"
        );
        assert!(
            engine.centroids[0][i] >= 0.0,
            "Centroid components must remain non-negative"
        );
    }

    // Test inference after 10,000 cycles
    let res = engine.infer(&base_odor);
    assert_eq!(res.best_class, 0);
    assert!(!res.is_novel);
    assert!(res.similarity > 0.80);
}

#[test]
fn test_end_to_end_adaptive_pipeline_with_modbus_telemetry() {
    let config = AdaptiveNoseConfig::industrial_default();
    let mut engine = AdaptiveNoseEngine::<4, 3>::new(&config);

    let gas_class_1 = [1_000.0, 45_000.0, 48_000.0, 50_000.0];
    let gas_class_2 = [50_000.0, 1_200.0, 47_000.0, 49_000.0];
    let gas_class_3 = [49_000.0, 50_000.0, 800.0, 48_000.0];

    engine.train_sample(&gas_class_1, 0);
    engine.train_sample(&gas_class_2, 1);
    engine.train_sample(&gas_class_3, 2);

    // Infer gas 2
    let result = engine.infer(&gas_class_2);
    assert_eq!(result.best_class, 1);
    assert!(!result.is_novel);

    // Format to Modbus Telemetry
    let mut telemetry = engine.to_modbus_telemetry(&result, 2, 85);
    assert_eq!(telemetry.system_status, 1); // Normal
    assert_eq!(telemetry.detected_gas_class, 2); // Class 2
    assert_eq!(telemetry.latency_us, 2);
    assert_eq!(telemetry.drift_degradation_index, 85);

    // Query Modbus Slave for Gas Class and Confidence
    let slave = ModbusSlave::new(1);
    let mut req = [0x01, 0x03, 0x00, 0x02, 0x00, 0x02, 0x00, 0x00];
    let crc = bionose_core::modbus::calculate_crc16(&req[..6]);
    req[6] = (crc & 0xFF) as u8;
    req[7] = (crc >> 8) as u8;

    let mut tx_buf = [0u8; 32];
    let len = slave
        .process_frame(&req, &mut telemetry, &mut tx_buf)
        .expect("Processed");
    assert_eq!(len, 9); // [Slave, Func, ByteCount(4), RegHi, RegLo, RegHi, RegLo, CRC, CRC]
    assert_eq!(tx_buf[3], 0x00);
    assert_eq!(tx_buf[4], 0x02); // Gas Class = 2
}
