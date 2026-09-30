//! Modbus RTU / RS485 Industrial Slave Engine for BioNose Edge.
//!
//! Provides bare-metal (#![no_std], zero-alloc) Modbus RTU frame parsing
//! and response serialization for industrial SCADA, PLC, and switchgear integration.
//!
//! Register Map:
//! - 0x0001 (Holding): System Status (0: Init, 1: Normal, 2: Alarm, 3: Degraded/Drift)
//! - 0x0002 (Holding): Detected Gas Class (0: Clean Air, 1..6: Target Gases)
//! - 0x0003 (Holding): Confidence / Cosine Similarity (0..10000 = 0.00% .. 100.00%)
//! - 0x0004 (Holding): Anomaly / Novelty Flag (0: Known Odor, 1: Novel Odor)
//! - 0x0005 (Holding): Inference Latency (microseconds)
//! - 0x0006 (Holding): Active Kenyon Cells Count (0..K)
//! - 0x0007 (Holding): Sensor Drift Degradation Index (0..1000)
//! - 0x0010 (Holding): Command Register (Write 0x0001: Trigger Auto-Zero Baseline, 0x0002: Field Adaptation)

/// Modbus RTU Error / Exception Codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ModbusException {
    IllegalFunction = 0x01,
    IllegalDataAddress = 0x02,
    IllegalDataValue = 0x03,
    SlaveDeviceFailure = 0x04,
}

/// System telemetry data presented over Modbus RTU registers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BioNoseTelemetry {
    pub system_status: u16,
    pub detected_gas_class: u16,
    pub confidence_basis_points: u16, // 0..10000
    pub is_novel: bool,
    pub latency_us: u16,
    pub active_kc_count: u16,
    pub drift_degradation_index: u16,
    pub command_register: u16,
}

impl BioNoseTelemetry {
    pub const fn default() -> Self {
        Self {
            system_status: 1, // Normal
            detected_gas_class: 0,
            confidence_basis_points: 0,
            is_novel: false,
            latency_us: 25,
            active_kc_count: 0,
            drift_degradation_index: 0,
            command_register: 0,
        }
    }

    /// Reads a holding register by 1-based address (e.g. 0x0001 to 0x0010).
    pub fn read_register(&self, address: u16) -> Result<u16, ModbusException> {
        match address {
            0x0001 => Ok(self.system_status),
            0x0002 => Ok(self.detected_gas_class),
            0x0003 => Ok(self.confidence_basis_points),
            0x0004 => Ok(if self.is_novel { 1 } else { 0 }),
            0x0005 => Ok(self.latency_us),
            0x0006 => Ok(self.active_kc_count),
            0x0007 => Ok(self.drift_degradation_index),
            0x0010 => Ok(self.command_register),
            _ => Err(ModbusException::IllegalDataAddress),
        }
    }

    /// Writes a holding register by 1-based address.
    pub fn write_register(&mut self, address: u16, value: u16) -> Result<(), ModbusException> {
        match address {
            0x0010 => {
                self.command_register = value;
                Ok(())
            }
            // Read-only telemetry registers
            0x0001..=0x0007 => Err(ModbusException::IllegalDataValue),
            _ => Err(ModbusException::IllegalDataAddress),
        }
    }
}

/// Standard Modbus CRC16 calculation (polynomial 0xA001, initial 0xFFFF).
pub fn calculate_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if (crc & 0x0001) != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

/// Zero-allocation Modbus RTU slave protocol handler.
#[derive(Debug, Clone, Copy)]
pub struct ModbusSlave {
    pub slave_address: u8,
}

impl ModbusSlave {
    pub const fn new(slave_address: u8) -> Self {
        Self { slave_address }
    }

    /// Processes an incoming Modbus RTU request frame and writes the response into `out_buf`.
    ///
    /// Returns the number of response bytes written, or None if the request is not for this slave.
    pub fn process_frame(
        &self,
        rx_buf: &[u8],
        telemetry: &mut BioNoseTelemetry,
        tx_buf: &mut [u8],
    ) -> Option<usize> {
        // Minimum Modbus frame: [Slave, Func, Data..., CRC_lo, CRC_hi] = at least 4 bytes
        if rx_buf.len() < 4 {
            return None;
        }

        let slave_id = rx_buf[0];
        if slave_id != self.slave_address && slave_id != 0 {
            // Not addressed to us and not broadcast
            return None;
        }

        // Verify CRC16
        let len = rx_buf.len();
        let expected_crc = (rx_buf[len - 2] as u16) | ((rx_buf[len - 1] as u16) << 8);
        let actual_crc = calculate_crc16(&rx_buf[..len - 2]);
        if actual_crc != expected_crc {
            return None; // Corrupted frame, ignore silently per Modbus specification
        }

        let function_code = rx_buf[1];
        let mut response_len = 0;

        match function_code {
            // 0x03: Read Holding Registers
            0x03 => {
                if rx_buf.len() != 8 {
                    response_len = self.build_exception(
                        tx_buf,
                        function_code,
                        ModbusException::IllegalDataValue,
                    );
                } else {
                    let start_addr = ((rx_buf[2] as u16) << 8) | (rx_buf[3] as u16);
                    let reg_count = ((rx_buf[4] as u16) << 8) | (rx_buf[5] as u16);

                    if reg_count == 0 || reg_count > 16 {
                        response_len = self.build_exception(
                            tx_buf,
                            function_code,
                            ModbusException::IllegalDataValue,
                        );
                    } else {
                        let byte_count = (reg_count * 2) as u8;
                        if tx_buf.len() < (5 + byte_count as usize) {
                            response_len = self.build_exception(
                                tx_buf,
                                function_code,
                                ModbusException::SlaveDeviceFailure,
                            );
                        } else {
                            tx_buf[0] = self.slave_address;
                            tx_buf[1] = 0x03;
                            tx_buf[2] = byte_count;

                            let mut ok = true;
                            let mut offset = 3;
                            for i in 0..reg_count {
                                match telemetry.read_register(start_addr + i) {
                                    Ok(val) => {
                                        tx_buf[offset] = (val >> 8) as u8;
                                        tx_buf[offset + 1] = (val & 0xFF) as u8;
                                        offset += 2;
                                    }
                                    Err(exc) => {
                                        response_len =
                                            self.build_exception(tx_buf, function_code, exc);
                                        ok = false;
                                        break;
                                    }
                                }
                            }

                            if ok {
                                response_len = offset;
                                let crc = calculate_crc16(&tx_buf[..response_len]);
                                tx_buf[response_len] = (crc & 0xFF) as u8;
                                tx_buf[response_len + 1] = (crc >> 8) as u8;
                                response_len += 2;
                            }
                        }
                    }
                }
            }

            // 0x06: Write Single Register
            0x06 => {
                if rx_buf.len() != 8 {
                    response_len = self.build_exception(
                        tx_buf,
                        function_code,
                        ModbusException::IllegalDataValue,
                    );
                } else {
                    let addr = ((rx_buf[2] as u16) << 8) | (rx_buf[3] as u16);
                    let val = ((rx_buf[4] as u16) << 8) | (rx_buf[5] as u16);

                    match telemetry.write_register(addr, val) {
                        Ok(()) => {
                            // Normal echo response
                            tx_buf[..6].copy_from_slice(&rx_buf[..6]);
                            let crc = calculate_crc16(&tx_buf[..6]);
                            tx_buf[6] = (crc & 0xFF) as u8;
                            tx_buf[7] = (crc >> 8) as u8;
                            response_len = 8;
                        }
                        Err(exc) => {
                            response_len = self.build_exception(tx_buf, function_code, exc);
                        }
                    }
                }
            }

            // Unsupported function code
            _ => {
                response_len =
                    self.build_exception(tx_buf, function_code, ModbusException::IllegalFunction);
            }
        }

        Some(response_len)
    }

    fn build_exception(&self, tx_buf: &mut [u8], function_code: u8, exc: ModbusException) -> usize {
        tx_buf[0] = self.slave_address;
        tx_buf[1] = function_code | 0x80; // Exception flag
        tx_buf[2] = exc as u8;
        let crc = calculate_crc16(&tx_buf[..3]);
        tx_buf[3] = (crc & 0xFF) as u8;
        tx_buf[4] = (crc >> 8) as u8;
        5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16_standard_vector() {
        // Standard Modbus test vector: [0x01, 0x03, 0x00, 0x01, 0x00, 0x02] -> CRC = 0xCB95 (Lo: 0x95, Hi: 0xCB)
        let frame = [0x01, 0x03, 0x00, 0x01, 0x00, 0x02];
        let crc = calculate_crc16(&frame);
        assert_eq!(crc, 0xCB95);
    }

    #[test]
    fn test_read_holding_registers() {
        let slave = ModbusSlave::new(1);
        let mut telemetry = BioNoseTelemetry::default();
        telemetry.detected_gas_class = 3; // Ammonia
        telemetry.confidence_basis_points = 8450; // 84.50%
        telemetry.active_kc_count = 13;

        // Query: Read 3 registers starting at 0x0002 (GasClass, Confidence, Novelty)
        // Request: [0x01, 0x03, 0x00, 0x02, 0x00, 0x03]
        let mut req = [0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x00, 0x00];
        let crc = calculate_crc16(&req[..6]);
        req[6] = (crc & 0xFF) as u8;
        req[7] = (crc >> 8) as u8;

        let mut tx_buf = [0u8; 64];
        let len = slave
            .process_frame(&req, &mut telemetry, &mut tx_buf)
            .expect("Frame valid");

        // Response should be: [0x01, 0x03, 0x06, 0x00, 0x03, 0x21, 0x02, 0x00, 0x00, CRC_lo, CRC_hi]
        assert_eq!(len, 11);
        assert_eq!(tx_buf[0], 0x01);
        assert_eq!(tx_buf[1], 0x03);
        assert_eq!(tx_buf[2], 0x06); // 6 bytes of data
                                     // Register 0x0002 = 3
        assert_eq!(tx_buf[3], 0x00);
        assert_eq!(tx_buf[4], 0x03);
        // Register 0x0003 = 8450 (0x2102)
        assert_eq!(tx_buf[5], 0x21);
        assert_eq!(tx_buf[6], 0x02);
        // Register 0x0004 = 0 (Not novel)
        assert_eq!(tx_buf[7], 0x00);
        assert_eq!(tx_buf[8], 0x00);
    }

    #[test]
    fn test_write_command_register() {
        let slave = ModbusSlave::new(1);
        let mut telemetry = BioNoseTelemetry::default();

        // Write Register 0x0010 with Value 0x0001 (Auto-Zero trigger)
        let mut req = [0x01, 0x06, 0x00, 0x10, 0x00, 0x01, 0x00, 0x00];
        let crc = calculate_crc16(&req[..6]);
        req[6] = (crc & 0xFF) as u8;
        req[7] = (crc >> 8) as u8;

        let mut tx_buf = [0u8; 64];
        let len = slave
            .process_frame(&req, &mut telemetry, &mut tx_buf)
            .expect("Frame valid");

        assert_eq!(len, 8);
        assert_eq!(telemetry.command_register, 0x0001);
    }
}
