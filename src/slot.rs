use std::time::Duration;

use modbus::{Client as _, tcp};

use crate::der_ac::DerAcClient;
use crate::error::Error;
use crate::meter::MeterClient;

pub(crate) const SLOT1_BASE: u16 = 40121;
pub(crate) const SLOT_SPACING: u16 = 174;
/// Applied to connect, read, and write on the Modbus TCP socket so no call can block forever.
pub(crate) const TCP_TIMEOUT: Duration = Duration::from_secs(3);

/// Each SolarEdge inverter has three meter slots, each of which may be empty.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SlotNumber {
	One = 1,
	Two = 2,
	Three = 3,
}

impl SlotNumber {
	/// All possible [`SlotNumber`]s.  Useful for iterating.
	pub const ALL: [Self; 3] = [Self::One, Self::Two, Self::Three];

	pub(crate) fn base(self) -> u16 {
		SLOT1_BASE + (self as u16 - 1) * SLOT_SPACING
	}
}

fn is_empty_slot_error(e: &modbus::Error) -> bool {
	match e {
		modbus::Error::Io(io_err) => io_err.kind() == std::io::ErrorKind::WouldBlock,
		modbus::Error::Exception(_) => true,
		_ => false,
	}
}

/// Supported types of meter [`Slot`]s.  May be extended to add other types.
pub enum Slot {
	Meter(MeterClient),
	DerAc(DerAcClient),
	Empty,
}

impl Slot {
	/// Open a slot on an existing transport, reusing a connection established by the caller.
	///
	/// Useful when the caller already has an open Modbus TCP session (e.g. from `TcpClient`)
	/// and wants to avoid closing and reopening the connection, which can cause SolarEdge
	/// inverters to fail due to their single-session limitation.
	pub fn open_on_transport(mut transport: tcp::Transport, slot: SlotNumber) -> Result<Self, Error> {
		let base = slot.base();

		let did = match transport.read_holding_registers(base, 1) {
			Ok(regs) => regs[0],
			Err(e) if is_empty_slot_error(&e) => return Ok(Slot::Empty),
			Err(e) => return Err(Error::from(e)),
		};
		match did {
			1 => {
				// Common block present; the meter model block header follows at offset 67.
				let meter_did = match transport.read_holding_registers(base + 67, 1) {
					Ok(regs) => regs[0],
					Err(e) if is_empty_slot_error(&e) => return Ok(Slot::Empty),
					Err(e) => return Err(Error::from(e)),
				};
				if !(201..=204).contains(&meter_did) {
					return Ok(Slot::Empty);
				}
				// Some inverters populate the SunSpec model header for unoccupied
				// slots but leave data registers unresponsive. Verify a data register
				// (C_Manufacturer at offset 2) actually responds before committing.
				match transport.read_holding_registers(base + 2, 1) {
					Ok(_) => Ok(Slot::Meter(MeterClient::from_transport(transport, base))),
					Err(e) if is_empty_slot_error(&e) => Ok(Slot::Empty),
					Err(e) => Err(Error::from(e)),
				}
			}
			701 => Ok(Slot::DerAc(DerAcClient::from_transport(transport, base))),
			_ => Ok(Slot::Empty),
		}
	}

	/// Open a slot by creating a new TCP connection.
	pub fn open(address: &str, port: u16, device_id: u8, slot: SlotNumber) -> Result<Self, Error> {
		let transport = tcp::Transport::new_with_cfg(
			address,
			tcp::Config {
				tcp_port: port,
				modbus_uid: device_id,
				tcp_connect_timeout: Some(TCP_TIMEOUT),
				tcp_read_timeout: Some(TCP_TIMEOUT),
				tcp_write_timeout: Some(TCP_TIMEOUT),
				..Default::default()
			},
		)?;
		Self::open_on_transport(transport, slot)
	}
}
