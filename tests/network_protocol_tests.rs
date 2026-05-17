use systemconfiguration::{NetworkService, Preferences};

#[test]
fn network_protocols_expose_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let prefs = Preferences::new("systemconfiguration-rs.network-protocol-tests", None)?;
    let services = NetworkService::copy_all(&prefs);
    if let Some(protocol) = services
        .first()
        .and_then(|service| service.copy_protocols().into_iter().next())
    {
        let _ = protocol.protocol_type();
        let _ = protocol.is_enabled();
        let _ = protocol.configuration();
    }
    Ok(())
}
