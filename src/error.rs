use std::fmt;

#[derive(Debug)]
pub enum Error {
	/// The device reports that the requested register value or its scale factor is not implemented.
	NotImplemented,
	InvalidPhaseCountValue(u16),
	Modbus(modbus::Error),
	Io(std::io::Error),
	#[cfg(feature = "discover")]
	Mdns(mdns_sd::Error),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NotImplemented => write!(f, "Register value is not implemented"),
			Self::Modbus(e) => write!(f, "Modbus error: {e}"),
			Self::Io(e) => write!(f, "I/O error: {e}"),
			Self::InvalidPhaseCountValue(val) => write!(f, "Invalid phase count value: {val}"),
			#[cfg(feature = "discover")]
			Self::Mdns(e) => write!(f, "mDNS error: {e}"),
		}
	}
}

impl std::error::Error for Error {}

impl From<modbus::Error> for Error {
	fn from(e: modbus::Error) -> Self {
		Error::Modbus(e)
	}
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Self {
		Error::Io(e)
	}
}

#[cfg(feature = "discover")]
impl From<mdns_sd::Error> for Error {
	fn from(e: mdns_sd::Error) -> Self {
		Error::Mdns(e)
	}
}
