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
					tcp_connect_timeout: Some(crate::slot::TCP_TIMEOUT),
					tcp_read_timeout: Some(crate::slot::TCP_TIMEOUT),
					tcp_write_timeout: Some(crate::slot::TCP_TIMEOUT),
					..Default::default()
				},
			)?,
		})
	}

	/// Consume this client and return the underlying transport.
	///
	/// Use this to pass the existing TCP session to [`Slot::open_on_transport`], avoiding the
	/// close/reopen cycle that causes SolarEdge inverters to fail due to their single-session
	/// limitation.
	pub fn into_transport(self) -> tcp::Transport {
		self.client
	}

	#[cfg(feature = "discover")]
	pub fn new_from_host_info(host_info: &crate::SolaredgeHostInfo) -> Result<Self, Error> {
		Self::new(&host_info.address(), host_info.port, host_info.modbus_id)
	}

	/// Value = "SunS" (0x53756e53). Uniquely identifies this as a SunSpec MODBUS Map
	pub fn spec_id(&mut self) -> Result<String, Error> {
		Ok(self.read_register(SunspecRegister::C_SunSpec_ID)?.string())
	}

	/// Value = 0x0001. Uniquely identifies this as a SunSpec Common Model Block
	pub fn model_id(&mut self) -> Result<u16, Error> {
		Ok(self.read_register(SunspecRegister::C_SunSpec_DID)?.u16())
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
		Ok(self.read_register(SunspecRegister::C_DeviceAddress)?.u16())
	}

	pub fn phase_count(&mut self) -> Result<PhaseCount, Error> {
		let val = self.read_register(SunspecRegister::I_PhaseCount)?.u16();
		PhaseCount::from_u16(val).ok_or(Error::InvalidPhaseCountValue(val))
	}

	/// Amps AC Total Current value
	pub fn ac_current(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_Current, SunspecRegister::I_AC_Current_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Amps AC Phase A Current value
	pub fn ac_current_a(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_CurrentA, SunspecRegister::I_AC_Current_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Amps AC Phase B Current value
	pub fn ac_current_b(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_CurrentB, SunspecRegister::I_AC_Current_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Amps AC Phase C Current value
	pub fn ac_current_c(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_CurrentC, SunspecRegister::I_AC_Current_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Volts AC Voltage Phase AB value
	pub fn ac_voltage_ab(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageAB, SunspecRegister::I_AC_Voltage_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Volts AC Voltage Phase BC value
	pub fn ac_voltage_bc(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageBC, SunspecRegister::I_AC_Voltage_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Volts AC Voltage Phase CA value
	pub fn ac_voltage_ca(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageCA, SunspecRegister::I_AC_Voltage_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Volts AC Voltage Phase A to N value
	pub fn ac_voltage_an(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageAN, SunspecRegister::I_AC_Voltage_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Volts AC Voltage Phase B to N value
	pub fn ac_voltage_bn(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageBN, SunspecRegister::I_AC_Voltage_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Volts AC Voltage Phase C to N value
	pub fn ac_voltage_cn(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VoltageCN, SunspecRegister::I_AC_Voltage_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Watts AC Power value
	pub fn ac_power(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_Power, SunspecRegister::I_AC_Power_SF)?;
		Ok(val.i16().scaled(scale))
	}

	/// Hertz AC Frequency value
	pub fn ac_frequency(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_Frequency, SunspecRegister::I_AC_Frequency_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// VA Apparent Power
	pub fn ac_va(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VA, SunspecRegister::I_AC_VA_SF)?;
		Ok(val.i16().scaled(scale))
	}

	/// VAR Reactive Power
	pub fn ac_var(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_VAR2, SunspecRegister::I_AC_VAR_SF2)?;
		Ok(val.i16().scaled(scale))
	}

	/// Power Factor
	pub fn power_factor(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_PF1, SunspecRegister::I_AC_PF_SF1)?;
		Ok(val.i16().scaled(scale))
	}

	/// WattHours AC Lifetime Energy production
	pub fn energy_wh(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_AC_Energy_WH, SunspecRegister::I_AC_Energy_WH_SF)?;
		Ok(val.u32().scaled(scale))
	}

	/// Amps DC Current value
	pub fn dc_current(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_DC_Current, SunspecRegister::I_DC_Current_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Volts DC Voltage value
	pub fn dc_voltage(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_DC_Voltage, SunspecRegister::I_DC_Voltage_SF)?;
		Ok(val.u16().scaled(scale))
	}

	/// Watts DC Power value
	pub fn dc_power(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_DC_Power, SunspecRegister::I_DC_Power_SF)?;
		Ok(val.i16().scaled(scale))
	}

	/// Degrees C Heat Sink Temperature
	pub fn temp_sink(&mut self) -> Result<f64, Error> {
		let (val, scale) = self.read_register_with_scale(SunspecRegister::I_Temp_Sink, SunspecRegister::I_Temp_SF)?;
		Ok(val.i16().scaled(scale))
	}

	/// Operating State
	pub fn status(&mut self) -> Result<InverterStatus, Error> {
		let val = self.read_register(SunspecRegister::I_Status)?.u16();
		Ok(InverterStatus::from_u16(val))
	}

	/// Vendor-defined operating state and error codes. For error description, meaning, and troubleshooting, refer to the SolarEdge
	/// Installation Guide.
	pub fn status_vendor(&mut self) -> Result<u16, Error> {
		Ok(self.read_register(SunspecRegister::I_Status_Vendor)?.u16())
	}

	/// Vendor-defined operating state and error codes. For error description, meaning, and troubleshooting, refer to the SolarEdge
	/// Installation Guide, 16MSB for controller type (3x, 8x, 18x) and 16 LSB for error code.
	pub fn status_vendor4(&mut self) -> Result<u32, Error> {
		Ok(self.read_register(SunspecRegister::I_Status_Vendor4)?.u32())
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
	) -> modbus::Result<(Vec<u16>, i16)> {
		read_scaled(&mut self.client, register.address(), register.size(), scale_register.address())
	}
}

/// Read a data register and its scale-factor register, combining them into a single Modbus
/// request when the two registers are contiguous in the address space.
pub(crate) fn read_scaled(
	client: &mut tcp::Transport,
	reg_addr: u16,
	reg_size: u16,
	sf_addr: u16,
) -> modbus::Result<(Vec<u16>, i16)> {
	if reg_addr + reg_size == sf_addr {
		let mut combined = client.read_holding_registers(reg_addr, reg_size + 1)?;
		let sf = combined.pop().unwrap() as i16;
		Ok((combined, sf))
	} else if sf_addr + 1 == reg_addr {
		let mut combined = client.read_holding_registers(sf_addr, 1 + reg_size)?;
		let sf = combined.remove(0) as i16;
		Ok((combined, sf))
	} else {
		let data = client.read_holding_registers(reg_addr, reg_size)?;
		let sf = client.read_holding_registers(sf_addr, 1)?[0] as i16;
		Ok((data, sf))
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
	fn u16(self) -> u16;
	fn i16(self) -> i16;
	fn u32(self) -> u32;
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

	fn u16(self) -> u16 {
		self[0]
	}

	fn i16(self) -> i16 {
		self[0] as i16
	}

	fn u32(self) -> u32 {
		((self[0] as u32) << 16) | self[1] as u32
	}
}

pub(crate) trait Scaled {
	fn scaled(self, scale: i16) -> f64;
}

impl Scaled for i16 {
	fn scaled(self, scale: i16) -> f64 {
		let value = self as f64;
		value * 10f64.powi(i32::from(scale))
	}
}

impl Scaled for u16 {
	fn scaled(self, scale: i16) -> f64 {
		let value = self as f64;
		value * 10f64.powi(i32::from(scale))
	}
}

impl Scaled for u32 {
	fn scaled(self, scale: i16) -> f64 {
		let value = self as f64;
		value * 10f64.powi(i32::from(scale))
	}
}
