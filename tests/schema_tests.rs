use systemconfiguration::Schema;

#[test]
fn schema_catalog_contains_common_keys() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = Schema::catalog()?;
    assert!(catalog.all.len() >= 270);
    assert_eq!(
        catalog.components.get("network").map(String::as_str),
        Some("Network")
    );
    assert_eq!(
        catalog.entities.get("ipv4").map(String::as_str),
        Some("IPv4")
    );
    assert!(catalog.proxies.contains_key("http_proxy"));
    assert!(catalog.contains("kSCDynamicStoreDomainSetup"));
    assert!(catalog.get("kSCPropNetDNSSearchOrder").is_some());
    assert!(catalog.get("kSCEntNetFireWire").is_some());
    Ok(())
}
