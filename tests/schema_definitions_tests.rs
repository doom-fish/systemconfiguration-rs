use systemconfiguration::schema_definitions;

#[test]
fn schema_constants_have_expected_values() {
    assert_eq!(
        schema_definitions::SC_PROP_NET_DNS_DOMAIN_NAME,
        "DomainName"
    );
    assert_eq!(schema_definitions::SC_PREF_CURRENT_SET, "CurrentSet");
    assert_eq!(schema_definitions::SC_COMP_NETWORK, "Network");
    assert_eq!(schema_definitions::SC_STATUS_OK, 0);
    assert_eq!(schema_definitions::SC_BOND_STATUS_OK, 0);
    assert_eq!(
        schema_definitions::SC_NETWORK_INTERFACE_TYPE_ETHERNET,
        "Ethernet"
    );
    assert_eq!(schema_definitions::CN_NETWORK_INFO_KEY_SSID, "SSID");
}
