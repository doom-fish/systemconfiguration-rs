use systemconfiguration::{NetworkService, Preferences};

#[test]
fn network_services_can_be_enumerated() -> Result<(), Box<dyn std::error::Error>> {
    let prefs = Preferences::new("systemconfiguration-rs.network-services-tests", None)?;
    let services = NetworkService::copy_all(&prefs);
    if let Some(service) = services.first() {
        let _ = service.service_id()?;
        let _ = service.name()?;
        let _ = service.interface();
        let _ = service.copy_protocols();
    }
    Ok(())
}
