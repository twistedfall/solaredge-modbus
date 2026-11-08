use modbus::{Client as _, tcp};

use crate::error::Error;

pub struct TcpClient {
	client: tcp::Transport,
}

impl TcpClient {
	pub fn new(address: &str, port: u16, device_id: u8) -> Result<Self, Error> {
		Ok(TcpClient {
			client: tcp::Transport::new_with_cfg(
				address,
				tcp::Config {
					tcp_port: port,
					modbus_uid: device_id,
					..Default::default()
				},
			)?,
		})
	}

	#[cfg(feature = "discover")]
	/// Connects using the discovered addresses, trying alternatives on failure
	pub fn new_from_host_info(host_info: &crate::SolaredgeHostInfo) -> Result<Self, Error> {
		let mut last_error = None;
		for address in &host_info.addresses {
			match Self::new(&address.to_string(), host_info.port, host_info.modbus_id) {
				Ok(client) => return Ok(client),
				Err(error) => last_error = Some(error),
			}
		}
		match last_error {
			Some(error) => Err(error),
			None => Self::new(&host_info.hostname, host_info.port, host_info.modbus_id),
		}
	}

	/// Value = "SunS" (0x53756e53). Uniquely identifies this as a SunSpec MODBUS Map
	pub fn spec_id(&mut self) -> Result<String, Error> {
		Ok(self.read_register(SunspecRegister::C_SunSpec_ID)?.string())
	}

	/// Value = 0x0001. Uniquely identifies this as a SunSpec Common Model Block
	pub fn model_id(&mut self) -> Result<u16, Error> {
		self.read_register(SunspecRegister::C_SunSpec_DID)?.u16()
	}

	/// Value Registered with SunSpec = "SolarEdge "
	pub fn manufacturer(&mut self) -> Result<String, Error> {
		Ok(self.read_register(SunspecRegister::C_Manufacturer)?.string())
	}

	/// SolarEdge Specific Value
	pub fn model(&mut self) -> Result<String, Error> {
		Ok(self.read_register(SunspecRegister::C_Model)?.string())
	}

	/// SolarEdge Specific Value
	pub fn version(&mut self) -> Result<String, Error> {
		Ok(self.read_register(SunspecRegister::C_Version)?.string())
	}

	/// SolarEdge Unique Value
	pub fn serial_number(&mut self) -> Result<String, Error> {
		Ok(self.read_register(SunspecRegister::C_SerialNumber)?.string())
	}

	/// MODBUS Unit ID
	pub fn device_address(&mut self) -> Result<u16, Error> {
		self.read_register(SunspecRegister::C_DeviceAddress)?.u16()
	}

	pub fn phase_count(&mut self) -> Result<PhaseCount, Error> {
		let val = self.read_register(SunspecRegister::I_PhaseCount)?.u16()?;
		PhaseCount::from_u16(val).ok_or(Error::InvalidPhaseCountValue(val))
	}

	/// Amps AC Total Current value
	pub fn ac_current(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_Current, SunspecRegister::I_AC_Current_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Amps AC Phase A Current value
	pub fn ac_current_a(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_CurrentA, SunspecRegister::I_AC_Current_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Amps AC Phase B Current value
	pub fn ac_current_b(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_CurrentB, SunspecRegister::I_AC_Current_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Amps AC Phase C Current value
	pub fn ac_current_c(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_CurrentC, SunspecRegister::I_AC_Current_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Volts AC Voltage Phase AB value
	pub fn ac_voltage_ab(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageAB, SunspecRegister::I_AC_Voltage_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Volts AC Voltage Phase BC value
	pub fn ac_voltage_bc(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageBC, SunspecRegister::I_AC_Voltage_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Volts AC Voltage Phase CA value
	pub fn ac_voltage_ca(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageCA, SunspecRegister::I_AC_Voltage_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Volts AC Voltage Phase A to N value
	pub fn ac_voltage_an(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageAN, SunspecRegister::I_AC_Voltage_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Volts AC Voltage Phase B to N value
	pub fn ac_voltage_bn(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageBN, SunspecRegister::I_AC_Voltage_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Volts AC Voltage Phase C to N value
	pub fn ac_voltage_cn(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageCN, SunspecRegister::I_AC_Voltage_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Watts AC Power value
	pub fn ac_power(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_Power, SunspecRegister::I_AC_Power_SF)?;
		val.i16()?.scaled(scale)
	}

	/// Hertz AC Frequency value
	pub fn ac_frequency(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_Frequency, SunspecRegister::I_AC_Frequency_SF)?;
		val.u16()?.scaled(scale)
	}

	/// VA Apparent Power
	pub fn ac_va(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VA, SunspecRegister::I_AC_VA_SF)?;
		val.i16()?.scaled(scale)
	}

	/// VAR Reactive Power
	pub fn ac_var(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VAR2, SunspecRegister::I_AC_VAR_SF2)?;
		val.i16()?.scaled(scale)
	}

	/// Power Factor
	pub fn power_factor(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_PF1, SunspecRegister::I_AC_PF_SF1)?;
		val.i16()?.scaled(scale)
	}

	/// WattHours AC Lifetime Energy production
	pub fn energy_wh(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_Energy_WH, SunspecRegister::I_AC_Energy_WH_SF)?;
		val.acc32().scaled(scale)
	}

	/// Amps DC Current value
	pub fn dc_current(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_DC_Current, SunspecRegister::I_DC_Current_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Volts DC Voltage value
	pub fn dc_voltage(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_DC_Voltage, SunspecRegister::I_DC_Voltage_SF)?;
		val.u16()?.scaled(scale)
	}

	/// Watts DC Power value
	pub fn dc_power(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_DC_Power, SunspecRegister::I_DC_Power_SF)?;
		val.i16()?.scaled(scale)
	}

	/// Degrees C Heat Sink Temperature
	pub fn temp_sink(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_Temp_Sink, SunspecRegister::I_Temp_SF)?;
		val.i16()?.scaled(scale)
	}

	/// Operating State
	pub fn status(&mut self) -> Result<InverterStatus, Error> {
		let val = self.read_register(SunspecRegister::I_Status)?.u16()?;
		Ok(InverterStatus::from_u16(val))
	}

	/// Vendor-defined operating state and error codes. For error description, meaning, and troubleshooting, refer to the SolarEdge
	/// Installation Guide.
	pub fn status_vendor(&mut self) -> Result<u16, Error> {
		self.read_register(SunspecRegister::I_Status_Vendor)?.u16()
	}

	/// Vendor-defined operating state and error codes. For error description, meaning, and troubleshooting, refer to the SolarEdge
	/// Installation Guide, 16MSB for controller type (3x, 8x, 18x) and 16 LSB for error code.
	pub fn status_vendor4(&mut self) -> Result<u32, Error> {
		self.read_register(SunspecRegister::I_Status_Vendor4)?.u32()
	}

	#[inline(always)]
	fn read_register<R: Register>(&mut self, register: R) -> modbus::Result<Vec<u16>> {
		self.client.read_holding_registers(register.address(), register.size())
	}

	/// Reads scale and the register in one call if they are adjacent.
	///
	/// This is an optimization to reduce the number of Modbus calls. If the register and scale are not adjacent, it falls back to reading them separately.
	#[inline(always)]
	fn read_register_with_scale<R: Register, RS: Register>(
		&mut self,
		register: R,
		scale_register: RS,
	) -> Result<(Vec<u16>, i16), Error> {
		if register.address() + register.size() == scale_register.address() {
			let mut combined = self
				.client
				.read_holding_registers(register.address(), register.size() + scale_register.size())?;
			let scale = combined.pop().unwrap();
			Ok((combined, scale as i16))
		} else if scale_register.address() + scale_register.size() == register.address() {
			let mut combined = self
				.client
				.read_holding_registers(scale_register.address(), scale_register.size() + register.size())?;
			let scale = combined.remove(0);
			Ok((combined, scale as i16))
		} else {
			Ok((self.read_register(register)?, self.read_register(scale_register)?.i16()?))
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum PhaseCount {
	SinglePhase,
	SplitPhase,
	ThreePhase,
}

impl PhaseCount {
	pub fn from_u16(value: u16) -> Option<Self> {
		match value {
			101 => Some(PhaseCount::SinglePhase),
			102 => Some(PhaseCount::SplitPhase),
			103 => Some(PhaseCount::ThreePhase),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum InverterStatus {
	Off,
	/// Sleeping (auto-shutdown) – Night mode
	Sleeping,
	/// Grid Monitoring/wake-up
	Starting,
	/// Inverter is ON and producing power
	Mppt,
	/// Production (curtailed)
	Throttled,
	ShuttingDown,
	Fault,
	/// Maintenance/setup
	Standby,
	/// Vendor-specific status, refer to SolarEdge Installation Guide
	Vendor(u16),
}

impl InverterStatus {
	pub fn from_u16(value: u16) -> Self {
		match value {
			1 => InverterStatus::Off,
			2 => InverterStatus::Sleeping,
			3 => InverterStatus::Starting,
			4 => InverterStatus::Mppt,
			5 => InverterStatus::Throttled,
			6 => InverterStatus::ShuttingDown,
			7 => InverterStatus::Fault,
			8 => InverterStatus::Standby,
			v => InverterStatus::Vendor(v),
		}
	}
}

trait Register {
	fn address(&self) -> u16;
	fn size(&self) -> u16;
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum SunspecRegister {
	/// Value = "SunS" (0x53756e53). Uniquely identifies this as a SunSpec MODBUS Map
	C_SunSpec_ID = 40000,
	/// Value = 0x0001. Uniquely identifies this as a SunSpec Common Model Block
	C_SunSpec_DID = 40002,
	/// Value Registered with SunSpec = "SolarEdge "
	C_Manufacturer = 40004,
	/// SolarEdge Specific Value
	C_Model = 40020,
	/// SolarEdge Specific Value
	C_Version = 40044,
	/// SolarEdge Unique Value
	C_SerialNumber = 40052,
	/// MODBUS Unit ID
	C_DeviceAddress = 40068,
	/// 101 = single phase
	/// 102 = split phase
	/// 103 = three phase
	I_PhaseCount = 40069,
	/// Amps AC Total Current value
	I_AC_Current = 40071,
	/// Amps AC Phase A Current value
	I_AC_CurrentA = 40072,
	/// Amps AC Phase B Current value
	I_AC_CurrentB = 40073,
	/// Amps AC Phase C Current value
	I_AC_CurrentC = 40074,
	/// AC Current scale factor
	I_AC_Current_SF = 40075,
	/// Volts AC Voltage Phase AB value
	I_AC_VoltageAB = 40076,
	/// Volts AC Voltage Phase BC value
	I_AC_VoltageBC = 40077,
	/// Volts AC Voltage Phase CA value
	I_AC_VoltageCA = 40078,
	/// Volts AC Voltage Phase A to N value
	I_AC_VoltageAN = 40079,
	/// Volts AC Voltage Phase B to N value
	I_AC_VoltageBN = 40080,
	/// Volts AC Voltage Phase C to N value
	I_AC_VoltageCN = 40081,
	/// AC Voltage scale factor
	I_AC_Voltage_SF = 40082,
	/// Watts AC Power value
	I_AC_Power = 40083,
	/// AC Power scale factor
	I_AC_Power_SF = 40084,
	/// Hertz AC Frequency value
	I_AC_Frequency = 40085,
	/// Scale factor
	I_AC_Frequency_SF = 40086,
	/// VA Apparent Power
	I_AC_VA = 40087,
	/// Scale factor
	I_AC_VA_SF = 40088,
	/// VAR Reactive Power
	I_AC_VAR2 = 40089,
	/// Scale factor
	I_AC_VAR_SF2 = 40090,
	/// Power Factor
	I_AC_PF1 = 40091,
	/// Scale factor
	I_AC_PF_SF1 = 40092,
	/// WattHours AC Lifetime Energy production
	I_AC_Energy_WH = 40093,
	/// Scale factor
	I_AC_Energy_WH_SF = 40095,
	/// Amps DC Current value
	I_DC_Current = 40096,
	/// Scale factor
	I_DC_Current_SF = 40097,
	/// Volts DC Voltage value
	I_DC_Voltage = 40098,
	/// Scale factor
	I_DC_Voltage_SF = 40099,
	/// Watts DC Power value
	I_DC_Power = 40100,
	/// Scale factor
	I_DC_Power_SF = 40101,
	/// Degrees C Heat Sink Temperature
	I_Temp_Sink = 40103,
	/// Scale factor
	I_Temp_SF = 40106,
	/// Operating State
	I_Status = 40107,
	/// Vendor-defined operating state and error codes. For error description, meaning, and troubleshooting, refer to the SolarEdge
	/// Installation Guide.
	I_Status_Vendor = 40108,
	/// Vendor-defined operating state and error codes. For error description, meaning, and troubleshooting, refer to the SolarEdge
	/// Installation Guide, 16MSB for controller type (3x, 8x, 18x) and 16 LSB for error code.
	I_Status_Vendor4 = 40119,
}

impl Register for SunspecRegister {
	#[inline(always)]
	fn address(&self) -> u16 {
		*self as u16
	}

	#[inline(always)]
	fn size(&self) -> u16 {
		match self {
			SunspecRegister::C_SunSpec_ID => 2,
			SunspecRegister::C_SunSpec_DID => 1,
			SunspecRegister::C_Manufacturer => 16,
			SunspecRegister::C_Model => 16,
			SunspecRegister::C_Version => 8,
			SunspecRegister::C_SerialNumber => 16,
			SunspecRegister::C_DeviceAddress => 1,
			SunspecRegister::I_PhaseCount => 1,
			SunspecRegister::I_AC_Current => 1,
			SunspecRegister::I_AC_CurrentA => 1,
			SunspecRegister::I_AC_CurrentB => 1,
			SunspecRegister::I_AC_CurrentC => 1,
			SunspecRegister::I_AC_Current_SF => 1,
			SunspecRegister::I_AC_VoltageAB => 1,
			SunspecRegister::I_AC_VoltageBC => 1,
			SunspecRegister::I_AC_VoltageCA => 1,
			SunspecRegister::I_AC_VoltageAN => 1,
			SunspecRegister::I_AC_VoltageBN => 1,
			SunspecRegister::I_AC_VoltageCN => 1,
			SunspecRegister::I_AC_Voltage_SF => 1,
			SunspecRegister::I_AC_Power => 1,
			SunspecRegister::I_AC_Power_SF => 1,
			SunspecRegister::I_AC_Frequency => 1,
			SunspecRegister::I_AC_Frequency_SF => 1,
			SunspecRegister::I_AC_VA => 1,
			SunspecRegister::I_AC_VA_SF => 1,
			SunspecRegister::I_AC_VAR2 => 1,
			SunspecRegister::I_AC_VAR_SF2 => 1,
			SunspecRegister::I_AC_PF1 => 1,
			SunspecRegister::I_AC_PF_SF1 => 1,
			SunspecRegister::I_AC_Energy_WH => 2,
			SunspecRegister::I_AC_Energy_WH_SF => 1,
			SunspecRegister::I_DC_Current => 1,
			SunspecRegister::I_DC_Current_SF => 1,
			SunspecRegister::I_DC_Voltage => 1,
			SunspecRegister::I_DC_Voltage_SF => 1,
			SunspecRegister::I_DC_Power => 1,
			SunspecRegister::I_DC_Power_SF => 1,
			SunspecRegister::I_Temp_Sink => 1,
			SunspecRegister::I_Temp_SF => 1,
			SunspecRegister::I_Status => 1,
			SunspecRegister::I_Status_Vendor => 1,
			SunspecRegister::I_Status_Vendor4 => 2,
		}
	}
}

trait RegisterResponse {
	fn string(self) -> String;
	fn u16(self) -> Result<u16, Error>;
	fn i16(self) -> Result<i16, Error>;
	fn u32(self) -> Result<u32, Error>;
	fn acc32(self) -> u32;
}

impl RegisterResponse for Vec<u16> {
	fn string(self) -> String {
		self
			.into_iter()
			.flat_map(|s| s.to_be_bytes())
			.take_while(|&b| b != 0)
			.map(char::from)
			.collect()
	}

	fn u16(self) -> Result<u16, Error> {
		match self[0] {
			u16::MAX => Err(Error::NotImplemented),
			value => Ok(value),
		}
	}

	fn i16(self) -> Result<i16, Error> {
		match self[0] as i16 {
			i16::MIN => Err(Error::NotImplemented),
			value => Ok(value),
		}
	}

	fn u32(self) -> Result<u32, Error> {
		match self.acc32() {
			u32::MAX => Err(Error::NotImplemented),
			value => Ok(value),
		}
	}

	fn acc32(self) -> u32 {
		// Accumulators permit the full u32 range, including 0 and u32::MAX.
		((self[0] as u32) << 16) | self[1] as u32
	}
}

trait Scaled {
	fn scaled(self, scale: i16) -> Result<f64, Error>;
}

impl<T: Into<f64>> Scaled for T {
	fn scaled(self, scale: i16) -> Result<f64, Error> {
		if scale == i16::MIN {
			return Err(Error::NotImplemented);
		}
		Ok(self.into() * 10f64.powi(i32::from(scale)))
	}
}

#[cfg(test)]
mod tests {
	use std::io::{Read, Write};
	use std::net::TcpListener;
	use std::thread;
	use std::time::Duration;

	use super::*;

	#[test]
	fn numeric_decoding_rejects_only_the_type_specific_sentinel() {
		assert!(matches!(vec![0xffff].u16(), Err(Error::NotImplemented)));
		assert_eq!(vec![0x8000].u16().unwrap(), 32768);
		assert_eq!(vec![0xfffe].u16().unwrap(), 65534);
		assert!(matches!(vec![0x8000].i16(), Err(Error::NotImplemented)));
		assert_eq!(vec![0x8001].i16().unwrap(), -32767);
		assert_eq!(vec![0xffff].i16().unwrap(), -1);
		assert!(matches!(vec![0xffff, 0xffff].u32(), Err(Error::NotImplemented)));
		assert_eq!(vec![0xffff, 0xfffe].u32().unwrap(), u32::MAX - 1);
		assert_eq!(vec![0, 0].acc32(), 0);
		assert_eq!(vec![0xffff, 0xffff].acc32(), u32::MAX);
	}

	#[test]
	fn scaling_rejects_unavailable_scale_and_preserves_signed_scales() {
		assert!(matches!(1u16.scaled(i16::MIN), Err(Error::NotImplemented)));
		assert!(matches!((-1i16).scaled(i16::MIN), Err(Error::NotImplemented)));
		assert!(matches!(0u32.scaled(i16::MIN), Err(Error::NotImplemented)));
		assert_eq!(123u16.scaled(-1).unwrap(), 12.3);
		assert_eq!((-123i16).scaled(-1).unwrap(), -12.3);
		assert_eq!(123u32.scaled(2).unwrap(), 12300.0);
	}

	fn mock_client(responses: Vec<(u16, Vec<u16>)>) -> (TcpClient, thread::JoinHandle<()>) {
		let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
		let port = listener.local_addr().unwrap().port();
		let server = thread::spawn(move || {
			let (mut stream, _) = listener.accept().unwrap();
			stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
			stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
			for (address, values) in responses {
				let mut request = [0u8; 12];
				stream.read_exact(&mut request).unwrap();
				assert_eq!(request[7], 3); // Read holding registers.
				assert_eq!(u16::from_be_bytes([request[8], request[9]]), address);
				assert_eq!(usize::from(u16::from_be_bytes([request[10], request[11]])), values.len());
				let byte_count = (values.len() * 2) as u8;
				let mut response = request[..4].to_vec();
				response.extend_from_slice(&(u16::from(byte_count) + 3).to_be_bytes());
				response.extend_from_slice(&[request[6], 3, byte_count]);
				for value in values {
					response.extend_from_slice(&value.to_be_bytes());
				}
				stream.write_all(&response).unwrap();
			}
		});
		(TcpClient::new("127.0.0.1", port, 1).unwrap(), server)
	}

	#[test]
	fn getters_report_unavailable_values_and_scales() {
		let (mut client, server) = mock_client(vec![
			(40002, vec![0xffff]),         // Common model ID.
			(40069, vec![0xffff]),         // Inverter model ID.
			(40107, vec![0xffff]),         // Status.
			(40108, vec![0xffff]),         // Vendor status.
			(40119, vec![0xffff, 0xffff]), // 32-bit vendor status.
			(40074, vec![0xffff, 0]),      // Unsigned measurement with adjacent scale.
			(40083, vec![0x8000, 0]),      // Signed measurement with adjacent scale.
			(40083, vec![123, 0x8000]),    // Unavailable adjacent scale.
			(40071, vec![123]),            // Measurement and scale in separate requests.
			(40075, vec![0x8000]),
			(40103, vec![0x8000]), // Unavailable measurement with separate valid scale.
			(40106, vec![0]),
			(40083, vec![(-123i16) as u16, (-1i16) as u16]),
			(40093, vec![0, 0, 0]), // Zero and maximum are valid accumulator values.
			(40093, vec![0xffff, 0xffff, 0]),
			(40093, vec![0, 1, 0x8000]), // Accumulator with unavailable scale.
		]);
		assert!(matches!(client.model_id(), Err(Error::NotImplemented)));
		assert!(matches!(client.phase_count(), Err(Error::NotImplemented)));
		assert!(matches!(client.status(), Err(Error::NotImplemented)));
		assert!(matches!(client.status_vendor(), Err(Error::NotImplemented)));
		assert!(matches!(client.status_vendor4(), Err(Error::NotImplemented)));
		assert!(matches!(client.ac_current_c(), Err(Error::NotImplemented)));
		assert!(matches!(client.ac_power(), Err(Error::NotImplemented)));
		assert!(matches!(client.ac_power(), Err(Error::NotImplemented)));
		assert!(matches!(client.ac_current(), Err(Error::NotImplemented)));
		assert!(matches!(client.temp_sink(), Err(Error::NotImplemented)));
		assert_eq!(client.ac_power().unwrap(), -12.3);
		assert_eq!(client.energy_wh().unwrap(), 0.0);
		assert_eq!(client.energy_wh().unwrap(), f64::from(u32::MAX));
		assert!(matches!(client.energy_wh(), Err(Error::NotImplemented)));
		server.join().unwrap();
	}
}
