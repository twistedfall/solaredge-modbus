use modbus::{Client as _, tcp};

use crate::error::Error;
use crate::slot::{SlotNumber, TCP_TIMEOUT};

/// Client for SunSpec model 701 (DER AC Measurement) slot in a SolarEdge inverter.
///
/// Register layout: [SunSpec model
/// 701](https://github.com/sunspec/models/blob/master/json/model_701.json).  SolarEdge-specific
/// implementation details: [SunSpec Implementation Technical
/// Note](https://knowledge-center.solaredge.com/sites/kc/files/sunspec-implementation-technical-note.pdf).
pub struct DerAcClient {
	client: tcp::Transport,
	base: u16,
}

impl DerAcClient {
	/// Create a [`DerAcClient`] with a new connection.  Connects to the meter in `slot` of the
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

	// --- Status ---

	pub fn state(&mut self) -> Result<u16, Error> {
		Ok(self.read(DerAcReg::St)?.u16())
	}

	pub fn inverter_state(&mut self) -> Result<Option<u16>, Error> {
		let v = self.read(DerAcReg::InvSt)?.u16();
		Ok(if v == u16::MAX {
			None
		} else {
			Some(v)
		})
	}

	pub fn connection_status(&mut self) -> Result<u16, Error> {
		Ok(self.read(DerAcReg::ConnSt)?.u16())
	}

	pub fn alarms(&mut self) -> Result<u32, Error> {
		Ok(self.read(DerAcReg::Alrm)?.u32())
	}

	pub fn der_mode(&mut self) -> Result<Option<u32>, Error> {
		let v = self.read(DerAcReg::DERMode)?.u32();
		Ok(if v == u32::MAX {
			None
		} else {
			Some(v)
		})
	}

	pub fn throttle_pct(&mut self) -> Result<Option<u16>, Error> {
		let v = self.read(DerAcReg::ThrotPct)?.u16();
		Ok(if v == u16::MAX {
			None
		} else {
			Some(v)
		})
	}

	pub fn throttle_source(&mut self) -> Result<Option<u32>, Error> {
		let v = self.read(DerAcReg::ThrotSrc)?.u32();
		Ok(if v == u32::MAX {
			None
		} else {
			Some(v)
		})
	}

	pub fn alarm_info(&mut self) -> Result<String, Error> {
		Ok(self.read(DerAcReg::MnAlrmInfo)?.string())
	}

	// --- AC totals ---

	pub fn ac_power(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::W, DerAcReg::W_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_va(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VA, DerAcReg::VA_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_var(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::Var, DerAcReg::Var_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_power_factor(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::PF, DerAcReg::PF_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_current(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::A, DerAcReg::A_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_voltage_ll(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::Llv, DerAcReg::V_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_voltage_ln(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::Lnv, DerAcReg::V_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_frequency(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::Hz, DerAcReg::Hz_SF)?;
		Ok(v.u32().scaled(sf))
	}

	// --- Energy totals ---

	pub fn total_wh_injected(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotWhInj, DerAcReg::TotWh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_wh_absorbed(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotWhAbs, DerAcReg::TotWh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_varh_injected(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotVarhInj, DerAcReg::TotVarh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_varh_absorbed(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotVarhAbs, DerAcReg::TotVarh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	// --- Temperatures ---

	pub fn temp_ambient(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TmpAmb, DerAcReg::Tmp_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn temp_cabinet(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TmpCab, DerAcReg::Tmp_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn temp_heatsink(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TmpSnk, DerAcReg::Tmp_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn temp_transformer(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TmpTrns, DerAcReg::Tmp_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn temp_switch(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TmpSw, DerAcReg::Tmp_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn temp_other(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TmpOt, DerAcReg::Tmp_SF)?;
		Ok(v.i16().scaled(sf))
	}

	// --- Per-phase L1 ---

	pub fn ac_power_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::WL1, DerAcReg::W_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_va_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VAL1, DerAcReg::VA_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_var_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VarL1, DerAcReg::Var_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_power_factor_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::PFL1, DerAcReg::PF_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_current_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::AL1, DerAcReg::A_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_voltage_l1_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VL1L2, DerAcReg::V_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_voltage_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VL1N, DerAcReg::V_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn total_wh_injected_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotWhInjL1, DerAcReg::TotWh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_wh_absorbed_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotWhAbsL1, DerAcReg::TotWh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_varh_injected_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotVarhInjL1, DerAcReg::TotVarh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_varh_absorbed_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotVarhAbsL1, DerAcReg::TotVarh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	// --- Per-phase L2 ---

	pub fn ac_power_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::WL2, DerAcReg::W_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_va_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VAL2, DerAcReg::VA_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_var_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VarL2, DerAcReg::Var_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_power_factor_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::PFL2, DerAcReg::PF_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_current_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::AL2, DerAcReg::A_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_voltage_l2_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VL2L3, DerAcReg::V_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_voltage_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VL2N, DerAcReg::V_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn total_wh_injected_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotWhInjL2, DerAcReg::TotWh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_wh_absorbed_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotWhAbsL2, DerAcReg::TotWh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_varh_injected_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotVarhInjL2, DerAcReg::TotVarh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_varh_absorbed_l2(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotVarhAbsL2, DerAcReg::TotVarh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	// --- Per-phase L3 ---

	pub fn ac_power_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::WL3, DerAcReg::W_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_va_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VAL3, DerAcReg::VA_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_var_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VarL3, DerAcReg::Var_SF)?;
		Ok(v.i16().scaled(sf))
	}

	pub fn ac_power_factor_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::PFL3, DerAcReg::PF_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_current_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::AL3, DerAcReg::A_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_voltage_l3_l1(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VL3L1, DerAcReg::V_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn ac_voltage_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::VL3N, DerAcReg::V_SF)?;
		Ok(v.u16().scaled(sf))
	}

	pub fn total_wh_injected_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotWhInjL3, DerAcReg::TotWh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_wh_absorbed_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotWhAbsL3, DerAcReg::TotWh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_varh_injected_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotVarhInjL3, DerAcReg::TotVarh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub fn total_varh_absorbed_l3(&mut self) -> Result<Option<f64>, Error> {
		let (v, sf) = self.read_register_with_scale(DerAcReg::TotVarhAbsL3, DerAcReg::TotVarh_SF)?;
		Ok(v.u64().scaled(sf))
	}

	pub(crate) fn from_transport(client: tcp::Transport, base: u16) -> Self {
		DerAcClient { client, base }
	}

	fn read_register_with_scale(
		&mut self,
		reg: DerAcReg,
		sf_reg: DerAcReg,
	) -> modbus::Result<(Vec<u16>, i16)> {
		crate::tcp_client::read_scaled(
			&mut self.client,
			self.base + reg.offset(),
			reg.size(),
			self.base + sf_reg.offset(),
		)
	}

	fn read(&mut self, reg: DerAcReg) -> modbus::Result<Vec<u16>> {
		self.client.read_holding_registers(self.base + reg.offset(), reg.size())
	}
}

// Offsets are from the slot base address.
// DID=701 is at offset 0, Length=153 at offset 1, data begins at offset 2.
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
enum DerAcReg {
	St = 3,
	InvSt = 4,
	ConnSt = 5,
	Alrm = 6,    // bitfield32, 2 regs
	DERMode = 8, // bitfield32, 2 regs
	W = 10,
	VA = 11,
	Var = 12,
	PF = 13,
	A = 14,
	Llv = 15,
	Lnv = 16,
	Hz = 17,         // uint32, 2 regs
	TotWhInj = 19,   // uint64, 4 regs
	TotWhAbs = 23,   // uint64, 4 regs
	TotVarhInj = 27, // uint64, 4 regs
	TotVarhAbs = 31, // uint64, 4 regs
	TmpAmb = 35,
	TmpCab = 36,
	TmpSnk = 37,
	TmpTrns = 38,
	TmpSw = 39,
	TmpOt = 40,
	// L1 group — +0..+2 int16, +3..+5 uint16, +6 uint16 (PPV = L-L voltage)
	WL1 = 41,          // int16
	VAL1 = 42,         // int16
	VarL1 = 43,        // int16
	PFL1 = 44,         // uint16
	AL1 = 45,          // uint16
	VL1N = 46,         // uint16, L1-N line-to-neutral voltage
	VL1L2 = 47,        // uint16, L1-L2 line-to-line voltage (PPV)
	TotWhInjL1 = 48,   // uint64, 4 regs
	TotWhAbsL1 = 52,   // uint64, 4 regs
	TotVarhInjL1 = 56, // uint64, 4 regs
	TotVarhAbsL1 = 60, // uint64, 4 regs
	// L2 group
	WL2 = 64,          // int16
	VAL2 = 65,         // int16
	VarL2 = 66,        // int16
	PFL2 = 67,         // uint16
	AL2 = 68,          // uint16
	VL2N = 69,         // uint16, L2-N line-to-neutral voltage
	VL2L3 = 70,        // uint16, L2-L3 line-to-line voltage (PPV)
	TotWhInjL2 = 71,   // uint64, 4 regs
	TotWhAbsL2 = 75,   // uint64, 4 regs
	TotVarhInjL2 = 79, // uint64, 4 regs
	TotVarhAbsL2 = 83, // uint64, 4 regs
	// L3 group
	WL3 = 87,           // int16
	VAL3 = 88,          // int16
	VarL3 = 89,         // int16
	PFL3 = 90,          // uint16
	AL3 = 91,           // uint16
	VL3N = 92,          // uint16, L3-N line-to-neutral voltage
	VL3L1 = 93,         // uint16, L3-L1 line-to-line voltage (PPV)
	TotWhInjL3 = 94,    // uint64, 4 regs
	TotWhAbsL3 = 98,    // uint64, 4 regs
	TotVarhInjL3 = 102, // uint64, 4 regs
	TotVarhAbsL3 = 106, // uint64, 4 regs
	ThrotPct = 110,
	ThrotSrc = 111, // bitfield32, 2 regs
	// Scale factors
	A_SF = 113,
	V_SF = 114,
	Hz_SF = 115,
	W_SF = 116,
	PF_SF = 117,
	VA_SF = 118,
	Var_SF = 119,
	TotWh_SF = 120,
	TotVarh_SF = 121,
	Tmp_SF = 122,
	MnAlrmInfo = 123, // string, 32 regs
}

impl DerAcReg {
	fn offset(self) -> u16 {
		self as u16
	}

	fn size(self) -> u16 {
		match self {
			DerAcReg::Alrm | DerAcReg::DERMode | DerAcReg::Hz | DerAcReg::ThrotSrc => 2,
			DerAcReg::TotWhInj
			| DerAcReg::TotWhAbs
			| DerAcReg::TotVarhInj
			| DerAcReg::TotVarhAbs
			| DerAcReg::TotWhInjL1
			| DerAcReg::TotWhAbsL1
			| DerAcReg::TotVarhInjL1
			| DerAcReg::TotVarhAbsL1
			| DerAcReg::TotWhInjL2
			| DerAcReg::TotWhAbsL2
			| DerAcReg::TotVarhInjL2
			| DerAcReg::TotVarhAbsL2
			| DerAcReg::TotWhInjL3
			| DerAcReg::TotWhAbsL3
			| DerAcReg::TotVarhInjL3
			| DerAcReg::TotVarhAbsL3 => 4,
			DerAcReg::MnAlrmInfo => 32,
			_ => 1,
		}
	}
}

trait Regs {
	fn string(self) -> String;
	fn u16(self) -> u16;
	fn i16(self) -> i16;
	fn u32(self) -> u32;
	fn u64(self) -> u64;
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

	fn u64(self) -> u64 {
		((self[0] as u64) << 48) | ((self[1] as u64) << 32) | ((self[2] as u64) << 16) | self[3] as u64
	}
}

trait Scaled {
	fn scaled(self, sf: i16) -> Option<f64>;
}

impl Scaled for i16 {
	fn scaled(self, sf: i16) -> Option<f64> {
		(self != i16::MIN).then(|| self as f64 * 10f64.powi(i32::from(sf)))
	}
}

impl Scaled for u16 {
	fn scaled(self, sf: i16) -> Option<f64> {
		(self != u16::MAX).then(|| self as f64 * 10f64.powi(i32::from(sf)))
	}
}

impl Scaled for u32 {
	fn scaled(self, sf: i16) -> Option<f64> {
		(self != u32::MAX).then(|| self as f64 * 10f64.powi(i32::from(sf)))
	}
}

impl Scaled for u64 {
	fn scaled(self, sf: i16) -> Option<f64> {
		(self != u64::MAX).then(|| self as f64 * 10f64.powi(i32::from(sf)))
	}
}
