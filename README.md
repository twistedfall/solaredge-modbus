# solaredge-modbus

[![Build Status](https://github.com/twistedfall/solaredge-modbus/actions/workflows/solaredge-modbus.yml/badge.svg)](https://github.com/twistedfall/solaredge-modbus/actions/workflows/solaredge-modbus.yml)
[![Documentation](https://docs.rs/solaredge-modbus/badge.svg)](https://docs.rs/solaredge-modbus)
[![Crates.io](https://img.shields.io/crates/v/solaredge-modbus)](https://crates.io/crates/solaredge-modbus)
![Maintenance](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

[Support the project](https://github.com/sponsors/twistedfall) | [Documentation](https://docs.rs/solaredge-modbus)


## Usage

Run:
```shell
cargo add solaredge-modbus
```
Or add to your Cargo.toml:
```toml
[dependencies]
solaredge-modbus = "0.1.2"
```

## SolarEdge modbus client for Rust

Enables access to the SolarEdge equipment over the local network via the
[Modbus protocol](https://knowledge-center.solaredge.com/sites/kc/files/sunspec-implementation-technical-note.pdf)
(solar panels, inverters, meters) with the nice typed Rust interface.

At the moment only the TCP transport is supported, as this is the only way I can test it, but I'm open for contributions to
add support for serial or other transports.

Modbus TCP might need to be enabled first, see for example
[this guide](https://github.com/binsentsu/home-assistant-solaredge-modbus#enabling-modbus-tcp-on-solaredge-inverter).

An optional `discover` feature is available to find the inverters on the local network using mDNS.
Enable it to use the following example:

```sh
cargo add solaredge-modbus --features discover
```

```rust
use solaredge_modbus::{TcpClient, discover_with_mdns};
use std::time::Duration;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
   let hosts = discover_with_mdns(Duration::from_secs(3), 1).await?;
   let host_info = hosts.into_iter().next().ok_or("No host found")?;
   let mut client = TcpClient::new_from_host_info(&host_info)?;
   let lifetime_energy = client.energy_wh()?;
   Ok(())
}
```

## License

LGPL-3.0
