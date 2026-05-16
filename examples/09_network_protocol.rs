use systemconfiguration::{NetworkService, Preferences};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prefs = Preferences::new("systemconfiguration-rs.network-protocol-example", None)?;
    if let Some(protocol) = NetworkService::copy_all(&prefs)
        .into_iter()
        .flat_map(|service| service.copy_protocols())
        .next()
    {
        println!(
            "protocol_type={:?} enabled={} has_config={}",
            protocol.protocol_type(),
            protocol.is_enabled(),
            protocol.configuration().is_some()
        );
    } else {
        println!("no network protocols available");
    }
    Ok(())
}
