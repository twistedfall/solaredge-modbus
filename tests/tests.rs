#[cfg(feature = "discover")]
use std::sync::{Mutex, OnceLock};
#[cfg(feature = "discover")]
use std::time::Duration;

#[cfg(feature = "discover")]
use solaredge_modbus::{Slot, SlotNumber, TcpClient, discover_with_mdns};

// SolarEdge enforces a single-session limit per inverter.  Serialize all
// hardware-touching tests so concurrent test threads don't collide.
#[cfg(feature = "discover")]
fn hw_lock() -> std::sync::MutexGuard<'static, ()> {
	static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
	LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[cfg(feature = "discover")]
#[tokio::test]
#[ignore] // requires a real inverter on the local network
async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
	let hosts = discover_with_mdns(Duration::from_secs(3), 1).await?;
	let host_info = hosts.into_iter().next().ok_or("No host found")?;
	let _guard = hw_lock();
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

// Discover all inverters and probe every slot, printing what each contains.
#[cfg(feature = "discover")]
#[tokio::test]
#[ignore] // requires inverters on the local network
async fn enumerate_slots() -> Result<(), Box<dyn std::error::Error>> {
	let hosts = discover_with_mdns(Duration::from_secs(5), 0).await?;
	if hosts.is_empty() {
		return Err("no inverters discovered".into());
	}
	let _guard = hw_lock();
	for host in &hosts {
		let addr = host.address();
		println!("=== {} ===", addr);
		for slot in SlotNumber::ALL {
			match Slot::open(addr.as_ref(), host.port, host.modbus_id, slot) {
				Ok(Slot::Meter(_)) => println!("  {slot:?}: Meter"),
				Ok(Slot::DerAc(_)) => println!("  {slot:?}: DerAc"),
				Ok(Slot::Empty) => println!("  {slot:?}: Empty"),
				Err(e) => println!("  {slot:?}: error ({e})"),
			}
		}
	}
	Ok(())
}

// Find the first Meter slot on any discovered inverter and exercise every method.
#[cfg(feature = "discover")]
#[tokio::test]
#[ignore] // requires an inverter with a meter slot on the local network
async fn meter_slot_all_reads() -> Result<(), Box<dyn std::error::Error>> {
	let hosts = discover_with_mdns(Duration::from_secs(5), 0).await?;
	let _guard = hw_lock();
	for host in &hosts {
		for slot in SlotNumber::ALL {
			let Ok(Slot::Meter(mut client)) = Slot::open(host.address().as_ref(), host.port, host.modbus_id, slot) else {
				continue;
			};

			dbg!(client.manufacturer()?);
			dbg!(client.model()?);
			dbg!(client.option()?);
			dbg!(client.version()?);
			dbg!(client.serial_number()?);
			dbg!(client.device_address()?);
			dbg!(client.events()?);

			dbg!(client.ac_current()?);
			dbg!(client.ac_current_a()?);
			dbg!(client.ac_current_b()?);
			dbg!(client.ac_current_c()?);

			dbg!(client.ac_voltage_ln()?);
			dbg!(client.ac_voltage_an()?);
			dbg!(client.ac_voltage_bn()?);
			dbg!(client.ac_voltage_cn()?);
			dbg!(client.ac_voltage_ll()?);
			dbg!(client.ac_voltage_ab()?);
			dbg!(client.ac_voltage_bc()?);
			dbg!(client.ac_voltage_ca()?);
			dbg!(client.ac_frequency()?);

			dbg!(client.ac_power()?);
			dbg!(client.ac_power_a()?);
			dbg!(client.ac_power_b()?);
			dbg!(client.ac_power_c()?);

			dbg!(client.ac_va()?);
			dbg!(client.ac_va_a()?);
			dbg!(client.ac_va_b()?);
			dbg!(client.ac_va_c()?);

			dbg!(client.ac_var()?);
			dbg!(client.ac_var_a()?);
			dbg!(client.ac_var_b()?);
			dbg!(client.ac_var_c()?);

			dbg!(client.power_factor()?);
			dbg!(client.power_factor_a()?);
			dbg!(client.power_factor_b()?);
			dbg!(client.power_factor_c()?);

			dbg!(client.exported_wh()?);
			dbg!(client.exported_wh_a()?);
			dbg!(client.exported_wh_b()?);
			dbg!(client.exported_wh_c()?);
			dbg!(client.imported_wh()?);
			dbg!(client.imported_wh_a()?);
			dbg!(client.imported_wh_b()?);
			dbg!(client.imported_wh_c()?);

			dbg!(client.exported_vah()?);
			dbg!(client.exported_vah_a()?);
			dbg!(client.exported_vah_b()?);
			dbg!(client.exported_vah_c()?);
			dbg!(client.imported_vah()?);
			dbg!(client.imported_vah_a()?);
			dbg!(client.imported_vah_b()?);
			dbg!(client.imported_vah_c()?);

			dbg!(client.imported_varh_q1()?);
			dbg!(client.imported_varh_q1_a()?);
			dbg!(client.imported_varh_q1_b()?);
			dbg!(client.imported_varh_q1_c()?);
			dbg!(client.imported_varh_q2()?);
			dbg!(client.imported_varh_q2_a()?);
			dbg!(client.imported_varh_q2_b()?);
			dbg!(client.imported_varh_q2_c()?);
			dbg!(client.exported_varh_q3()?);
			dbg!(client.exported_varh_q3_a()?);
			dbg!(client.exported_varh_q3_b()?);
			dbg!(client.exported_varh_q3_c()?);
			dbg!(client.exported_varh_q4()?);
			dbg!(client.exported_varh_q4_a()?);
			dbg!(client.exported_varh_q4_b()?);
			dbg!(client.exported_varh_q4_c()?);

			dbg!(client.grid_power_and_voltage()?);

			return Ok(());
		}
	}
	Err("no Meter slot found on any discovered inverter".into())
}

// Find the first DerAc slot on any discovered inverter and exercise every method.
#[cfg(feature = "discover")]
#[tokio::test]
#[ignore] // requires an inverter with a DER AC slot on the local network
async fn der_ac_slot_all_reads() -> Result<(), Box<dyn std::error::Error>> {
	let hosts = discover_with_mdns(Duration::from_secs(5), 0).await?;
	let _guard = hw_lock();
	for host in &hosts {
		for slot in SlotNumber::ALL {
			let Ok(Slot::DerAc(mut client)) = Slot::open(host.address().as_ref(), host.port, host.modbus_id, slot) else {
				continue;
			};

			dbg!(client.state()?);
			dbg!(client.inverter_state()?);
			dbg!(client.connection_status()?);
			dbg!(client.alarms()?);
			dbg!(client.der_mode()?);
			dbg!(client.alarm_info()?);

			dbg!(client.ac_power()?);
			dbg!(client.ac_va()?);
			dbg!(client.ac_var()?);
			dbg!(client.ac_power_factor()?);
			dbg!(client.ac_current()?);
			dbg!(client.ac_voltage_ll()?);
			dbg!(client.ac_voltage_ln()?);
			dbg!(client.ac_frequency()?);

			dbg!(client.total_wh_injected()?);
			dbg!(client.total_wh_absorbed()?);
			dbg!(client.total_varh_injected()?);
			dbg!(client.total_varh_absorbed()?);

			dbg!(client.temp_ambient()?);
			dbg!(client.temp_cabinet()?);
			dbg!(client.temp_heatsink()?);
			dbg!(client.temp_transformer()?);
			dbg!(client.temp_switch()?);
			dbg!(client.temp_other()?);

			dbg!(client.ac_power_l1()?);
			dbg!(client.ac_va_l1()?);
			dbg!(client.ac_var_l1()?);
			dbg!(client.ac_power_factor_l1()?);
			dbg!(client.ac_current_l1()?);
			dbg!(client.ac_voltage_l1_l2()?);
			dbg!(client.ac_voltage_l1()?);
			dbg!(client.total_wh_injected_l1()?);
			dbg!(client.total_wh_absorbed_l1()?);

			dbg!(client.ac_power_l2()?);
			dbg!(client.ac_va_l2()?);
			dbg!(client.ac_var_l2()?);
			dbg!(client.ac_power_factor_l2()?);
			dbg!(client.ac_current_l2()?);
			dbg!(client.ac_voltage_l2_l3()?);
			dbg!(client.ac_voltage_l2()?);
			dbg!(client.total_wh_injected_l2()?);
			dbg!(client.total_wh_absorbed_l2()?);

			dbg!(client.ac_power_l3()?);
			dbg!(client.ac_va_l3()?);
			dbg!(client.ac_var_l3()?);
			dbg!(client.ac_power_factor_l3()?);
			dbg!(client.ac_current_l3()?);
			dbg!(client.ac_voltage_l3_l1()?);
			dbg!(client.ac_voltage_l3()?);
			dbg!(client.total_wh_injected_l3()?);
			dbg!(client.total_wh_absorbed_l3()?);

			dbg!(client.throttle_pct()?);
			dbg!(client.throttle_source()?);

			return Ok(());
		}
	}
	Err("no DerAc slot found on any discovered inverter".into())
}
