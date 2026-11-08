#![cfg(feature = "discover")]

use std::time::Duration;

use solaredge_modbus::{TcpClient, discover_with_mdns};

#[tokio::test]
#[ignore] // requires a real inverter on the local network
async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
	let hosts = discover_with_mdns(Duration::from_secs(3), 1).await?;
	let host_info = hosts.into_iter().next().ok_or("No host found")?;
	let mut client = TcpClient::new_from_host_info(&host_info)?;
	dbg!(client.spec_id()?);
	dbg!(client.model_id()?);
	dbg!(client.manufacturer()?);
	dbg!(client.model()?);
	dbg!(client.version()?);
	dbg!(client.serial_number()?);
	dbg!(client.device_address()?);
	dbg!(client.phase_count()?);
	dbg!(client.ac_current()?);
	dbg!(client.ac_current_a()?);
	dbg!(client.ac_current_b()?);
	dbg!(client.ac_current_c()?);
	dbg!(client.ac_voltage_ab()?);
	dbg!(client.ac_voltage_bc()?);
	dbg!(client.ac_voltage_ca()?);
	dbg!(client.ac_voltage_an()?);
	dbg!(client.ac_voltage_bn()?);
	dbg!(client.ac_voltage_cn()?);
	dbg!(client.ac_power()?);
	dbg!(client.ac_frequency()?);
	dbg!(client.ac_va()?);
	dbg!(client.ac_var()?);
	dbg!(client.power_factor()?);
	dbg!(client.energy_wh()?);
	dbg!(client.dc_current()?);
	dbg!(client.dc_voltage()?);
	dbg!(client.dc_power()?);
	dbg!(client.temp_sink()?);
	dbg!(client.status()?);
	dbg!(client.status_vendor()?);
	dbg!(client.status_vendor4()?);

	Ok(())
}
