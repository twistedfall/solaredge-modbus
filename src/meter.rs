use modbus::{Client as _, tcp};

use crate::error::Error;
use crate::slot::{SlotNumber, TCP_TIMEOUT};

/// Client for a SunSpec AC meter slot in a SolarEdge inverter (models 201–204).
///
/// Register layout: [SunSpec model
/// 203](https://github.com/sunspec/models/blob/master/json/model_203.json).  SolarEdge-specific
/// implementation details: [SunSpec Implementation Technical
/// Note](https://knowledge-center.solaredge.com/sites/kc/files/sunspec-implementation-technical-note.pdf).
pub struct MeterClient {
	client: tcp::Transport,
	base: u16,
}

impl MeterClient {
	/// Create a [`MeterClient`] with a new connection.  Connects to the meter in `slot` of the
	/// inverter at `address` on `port` and with ID `device_id`.
	pub fn new(address: &str, port: u16, device_id: u8, slot: SlotNumber) -> Result<Self, Error> {
		Ok(Self::from_transport(
			tcp::Transport::new_with_cfg(
				address,
				tcp::Config {
					tcp_port: port,
					modbus_uid: device_id,
					tcp_connect_timeout: Some(TCP_TIMEOUT),
					tcp_read_timeout: Some(TCP_TIMEOUT),
					tcp_write_timeout: Some(TCP_TIMEOUT),
					..Default::default()
				},
			)?,
			slot.base(),
		))
	}

	pub fn dump_raw(&mut self, offset: u16, count: u16) -> Result<Vec<u16>, Error> {
		Ok(self.client.read_holding_registers(self.base + offset, count)?)
	}

	pub fn manufacturer(&mut self) -> Result<String, Error> {
		Ok(self.read(MeterReg::C_Manufacturer)?.string())
	}

	pub fn model(&mut self) -> Result<String, Error> {
		Ok(self.read(MeterReg::C_Model)?.string())
	}

	pub fn option(&mut self) -> Result<String, Error> {
		Ok(self.read(MeterReg::C_Option)?.string())
	}

	pub fn version(&mut self) -> Result<String, Error> {
		Ok(self.read(MeterReg::C_Version)?.string())
	}

	pub fn serial_number(&mut self) -> Result<String, Error> {
		Ok(self.read(MeterReg::C_SerialNumber)?.string())
	}

	pub fn device_address(&mut self) -> Result<u16, Error> {
		Ok(self.read(MeterReg::C_DeviceAddress)?.u16())
	}

	pub fn ac_current(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Current, MeterReg::M_AC_Current_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_current_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Current_A, MeterReg::M_AC_Current_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_current_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Current_B, MeterReg::M_AC_Current_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_current_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Current_C, MeterReg::M_AC_Current_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_voltage_ln(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Voltage_LN, MeterReg::M_AC_Voltage_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_voltage_an(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Voltage_AN, MeterReg::M_AC_Voltage_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_voltage_bn(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Voltage_BN, MeterReg::M_AC_Voltage_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_voltage_cn(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Voltage_CN, MeterReg::M_AC_Voltage_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_voltage_ll(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Voltage_LL, MeterReg::M_AC_Voltage_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_voltage_ab(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Voltage_AB, MeterReg::M_AC_Voltage_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_voltage_bc(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Voltage_BC, MeterReg::M_AC_Voltage_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_voltage_ca(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Voltage_CA, MeterReg::M_AC_Voltage_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_frequency(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Freq, MeterReg::M_AC_Freq_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_power(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Power, MeterReg::M_AC_Power_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_power_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Power_A, MeterReg::M_AC_Power_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_power_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Power_B, MeterReg::M_AC_Power_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_power_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_Power_C, MeterReg::M_AC_Power_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_va(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_VA, MeterReg::M_AC_VA_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_va_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_VA_A, MeterReg::M_AC_VA_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_va_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_VA_B, MeterReg::M_AC_VA_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_va_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_VA_C, MeterReg::M_AC_VA_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_var(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_VAR, MeterReg::M_AC_VAR_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_var_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_VAR_A, MeterReg::M_AC_VAR_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_var_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_VAR_B, MeterReg::M_AC_VAR_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_var_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_VAR_C, MeterReg::M_AC_VAR_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn power_factor(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_PF, MeterReg::M_AC_PF_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn power_factor_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_PF_A, MeterReg::M_AC_PF_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn power_factor_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_PF_B, MeterReg::M_AC_PF_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn power_factor_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_AC_PF_C, MeterReg::M_AC_PF_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn exported_wh(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Exported, MeterReg::M_Energy_W_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_wh_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Exported_A, MeterReg::M_Energy_W_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_wh_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Exported_B, MeterReg::M_Energy_W_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_wh_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Exported_C, MeterReg::M_Energy_W_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_wh(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Imported, MeterReg::M_Energy_W_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_wh_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Imported_A, MeterReg::M_Energy_W_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_wh_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Imported_B, MeterReg::M_Energy_W_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_wh_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Imported_C, MeterReg::M_Energy_W_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_vah(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Exported_VA, MeterReg::M_Energy_VA_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_vah_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Exported_VA_A, MeterReg::M_Energy_VA_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_vah_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Exported_VA_B, MeterReg::M_Energy_VA_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_vah_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Exported_VA_C, MeterReg::M_Energy_VA_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_vah(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Imported_VA, MeterReg::M_Energy_VA_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_vah_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Imported_VA_A, MeterReg::M_Energy_VA_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_vah_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Imported_VA_B, MeterReg::M_Energy_VA_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_vah_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Imported_VA_C, MeterReg::M_Energy_VA_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_varh_q1(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Import_VARh_Q1, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_varh_q1_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Import_VARh_Q1A, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_varh_q1_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Import_VARh_Q1B, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_varh_q1_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Import_VARh_Q1C, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_varh_q2(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Import_VARh_Q2, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_varh_q2_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Import_VARh_Q2A, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_varh_q2_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Import_VARh_Q2B, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn imported_varh_q2_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Import_VARh_Q2C, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_varh_q3(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Export_VARh_Q3, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_varh_q3_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Export_VARh_Q3A, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_varh_q3_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Export_VARh_Q3B, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_varh_q3_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Export_VARh_Q3C, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_varh_q4(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Export_VARh_Q4, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_varh_q4_a(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Export_VARh_Q4A, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_varh_q4_b(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Export_VARh_Q4B, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn exported_varh_q4_c(&mut self) -> Result<f64, Error> {
		let (v, sf) = self.read_register_with_scale(MeterReg::M_Export_VARh_Q4C, MeterReg::M_Energy_VAR_SF)?;
		Ok(v.u32().scaled(sf))
	}

	pub fn events(&mut self) -> Result<u32, Error> {
		Ok(self.read(MeterReg::M_Events)?.u32())
	}

	/// Read grid real power (W) and line-to-line voltage (V) in a single Modbus request.
	///
	/// Returns `(power, voltage)` where power is negative when exporting to the grid.
	pub fn grid_power_and_voltage(&mut self) -> Result<(f64, f64), Error> {
		// Read M_AC_Voltage_AB through M_AC_Power_SF contiguously (offsets 79–89, 11 registers).
		let regs = self
			.client
			.read_holding_registers(self.base + MeterReg::M_AC_Voltage_AB.offset(), 11)?;
		let voltage_sf = regs[3] as i16; // M_AC_Voltage_SF at offset 82
		let power_sf = regs[10] as i16; // M_AC_Power_SF at offset 89
		Ok(((regs[6] as i16).scaled(power_sf), (regs[0] as i16).scaled(voltage_sf)))
	}

	pub(crate) fn from_transport(client: tcp::Transport, base: u16) -> Self {
		MeterClient { client, base }
	}

	fn read_register_with_scale(&mut self, reg: MeterReg, sf_reg: MeterReg) -> modbus::Result<(Vec<u16>, i16)> {
		crate::tcp_client::read_scaled(
			&mut self.client,
			self.base + reg.offset(),
			reg.size(),
			self.base + sf_reg.offset(),
		)
	}

	fn read(&mut self, reg: MeterReg) -> modbus::Result<Vec<u16>> {
		self.client.read_holding_registers(self.base + reg.offset(), reg.size())
	}
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
enum MeterReg {
	C_Manufacturer = 2,
	C_Model = 18,
	C_Option = 34,
	C_Version = 42,
	C_SerialNumber = 50,
	C_DeviceAddress = 66,
	// Offsets 67-68 are the SunSpec model block header (DID=203, Length=105); data starts at 69.
	M_AC_Current = 69,
	M_AC_Current_A = 70,
	M_AC_Current_B = 71,
	M_AC_Current_C = 72,
	M_AC_Current_SF = 73,
	M_AC_Voltage_LN = 74,
	M_AC_Voltage_AN = 75,
	M_AC_Voltage_BN = 76,
	M_AC_Voltage_CN = 77,
	M_AC_Voltage_LL = 78,
	M_AC_Voltage_AB = 79,
	M_AC_Voltage_BC = 80,
	M_AC_Voltage_CA = 81,
	M_AC_Voltage_SF = 82,
	M_AC_Freq = 83,
	M_AC_Freq_SF = 84,
	M_AC_Power = 85,
	M_AC_Power_A = 86,
	M_AC_Power_B = 87,
	M_AC_Power_C = 88,
	M_AC_Power_SF = 89,
	M_AC_VA = 90,
	M_AC_VA_A = 91,
	M_AC_VA_B = 92,
	M_AC_VA_C = 93,
	M_AC_VA_SF = 94,
	M_AC_VAR = 95,
	M_AC_VAR_A = 96,
	M_AC_VAR_B = 97,
	M_AC_VAR_C = 98,
	M_AC_VAR_SF = 99,
	M_AC_PF = 100,
	M_AC_PF_A = 101,
	M_AC_PF_B = 102,
	M_AC_PF_C = 103,
	M_AC_PF_SF = 104,
	M_Exported = 105,
	M_Exported_A = 107,
	M_Exported_B = 109,
	M_Exported_C = 111,
	M_Imported = 113,
	M_Imported_A = 115,
	M_Imported_B = 117,
	M_Imported_C = 119,
	M_Energy_W_SF = 121,
	M_Exported_VA = 122,
	M_Exported_VA_A = 124,
	M_Exported_VA_B = 126,
	M_Exported_VA_C = 128,
	M_Imported_VA = 130,
	M_Imported_VA_A = 132,
	M_Imported_VA_B = 134,
	M_Imported_VA_C = 136,
	M_Energy_VA_SF = 138,
	M_Import_VARh_Q1 = 139,
	M_Import_VARh_Q1A = 141,
	M_Import_VARh_Q1B = 143,
	M_Import_VARh_Q1C = 145,
	M_Import_VARh_Q2 = 147,
	M_Import_VARh_Q2A = 149,
	M_Import_VARh_Q2B = 151,
	M_Import_VARh_Q2C = 153,
	M_Export_VARh_Q3 = 155,
	M_Export_VARh_Q3A = 157,
	M_Export_VARh_Q3B = 159,
	M_Export_VARh_Q3C = 161,
	M_Export_VARh_Q4 = 163,
	M_Export_VARh_Q4A = 165,
	M_Export_VARh_Q4B = 167,
	M_Export_VARh_Q4C = 169,
	M_Energy_VAR_SF = 171,
	M_Events = 173,
}

impl MeterReg {
	fn offset(self) -> u16 {
		self as u16
	}

	fn size(self) -> u16 {
		match self {
			MeterReg::C_Manufacturer => 16,
			MeterReg::C_Model => 16,
			MeterReg::C_Option => 8,
			MeterReg::C_Version => 8,
			MeterReg::C_SerialNumber => 16,
			MeterReg::M_Exported
			| MeterReg::M_Exported_A
			| MeterReg::M_Exported_B
			| MeterReg::M_Exported_C
			| MeterReg::M_Imported
			| MeterReg::M_Imported_A
			| MeterReg::M_Imported_B
			| MeterReg::M_Imported_C
			| MeterReg::M_Exported_VA
			| MeterReg::M_Exported_VA_A
			| MeterReg::M_Exported_VA_B
			| MeterReg::M_Exported_VA_C
			| MeterReg::M_Imported_VA
			| MeterReg::M_Imported_VA_A
			| MeterReg::M_Imported_VA_B
			| MeterReg::M_Imported_VA_C
			| MeterReg::M_Import_VARh_Q1
			| MeterReg::M_Import_VARh_Q1A
			| MeterReg::M_Import_VARh_Q1B
			| MeterReg::M_Import_VARh_Q1C
			| MeterReg::M_Import_VARh_Q2
			| MeterReg::M_Import_VARh_Q2A
			| MeterReg::M_Import_VARh_Q2B
			| MeterReg::M_Import_VARh_Q2C
			| MeterReg::M_Export_VARh_Q3
			| MeterReg::M_Export_VARh_Q3A
			| MeterReg::M_Export_VARh_Q3B
			| MeterReg::M_Export_VARh_Q3C
			| MeterReg::M_Export_VARh_Q4
			| MeterReg::M_Export_VARh_Q4A
			| MeterReg::M_Export_VARh_Q4B
			| MeterReg::M_Export_VARh_Q4C
			| MeterReg::M_Events => 2,
			_ => 1,
		}
	}
}

trait Regs {
	fn string(self) -> String;
	fn u16(self) -> u16;
	fn i16(self) -> i16;
	fn u32(self) -> u32;
}

impl Regs for Vec<u16> {
	fn string(self) -> String {
		self
			.into_iter()
			.flat_map(|w| w.to_be_bytes())
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

use crate::tcp_client::Scaled;
