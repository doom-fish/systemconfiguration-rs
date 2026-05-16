use systemconfiguration::Preferences;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prefs = Preferences::new("systemconfiguration-rs.network-services-example", None)?;
    for service in prefs.network_services() {
        println!(
            "id={:?} enabled={} name={:?} protocols={}",
            service.service_id()?,
            service.is_enabled(),
            service.name()?,
            service.copy_protocols().len()
        );
    }
    Ok(())
}
