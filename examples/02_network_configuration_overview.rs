use systemconfiguration::NetworkConfiguration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let overview = NetworkConfiguration::overview()?;
    println!(
        "interfaces={} services={} sets={} current={:?}",
        overview.interface_count,
        overview.service_count,
        overview.set_count,
        overview.current_set_name
    );
    Ok(())
}
