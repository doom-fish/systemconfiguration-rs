use systemconfiguration::NetworkConfiguration;

#[test]
fn network_configuration_overview_exposes_type_maps() -> Result<(), Box<dyn std::error::Error>> {
    let overview = NetworkConfiguration::overview()?;
    assert!(overview.interface_types.contains_key("Ethernet"));
    assert!(overview.protocol_types.contains_key("IPv4"));
    assert!(overview.interface_count > 0);
    Ok(())
}
