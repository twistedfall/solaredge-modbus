//! # SolarEdge modbus client for Rust
//!
//! Enables access to the SolarEdge equipment over the local network via the
//! [Modbus protocol](https://knowledge-center.solaredge.com/sites/kc/files/sunspec-implementation-technical-note.pdf)
//! (solar panels, inverters, meters) with the nice typed Rust interface.
//!
//! At the moment only the TCP transport is supported, as this is the only way I can test it, but I'm open for contributions to
//! add support for serial or other transports.
//!
//! Modbus TCP might need to be enabled first, see for example
//! [this guide](https://github.com/binsentsu/home-assistant-solaredge-modbus#enabling-modbus-tcp-on-solaredge-inverter).
//!
//! An optional `discover` feature is available to find the inverters on the local network using mDNS.
//!
//! ```no_run
//! # #[cfg(feature = "discover")]
//! use solaredge_modbus::{TcpClient, discover_with_mdns};
//! use std::time::Duration;
//!
//! # #[cfg(feature = "discover")]
//! async fn run() -> Result<(), Box<dyn std::error::Error>> {
//!    let hosts = discover_with_mdns(Duration::from_secs(3), 1).await?;
//!    let host_info = hosts.into_iter().next().ok_or("No host found")?;
//!    let mut client = TcpClient::new_from_host_info(&host_info)?;
//!    let lifetime_energy = client.energy_wh()?;
//!    Ok(())
//! }
//! ```

pub use der_ac::DerAcClient;
pub use error::Error;
pub use meter::MeterClient;
pub use slot::{Slot, SlotNumber};
pub use tcp_client::TcpClient;

pub mod der_ac;
mod error;
pub mod meter;
pub mod slot;
pub mod tcp_client;
#[cfg(feature = "discover")]
pub use discover::{SolaredgeHostInfo, discover_with_mdns};
#[cfg(feature = "discover")]
pub mod discover;
