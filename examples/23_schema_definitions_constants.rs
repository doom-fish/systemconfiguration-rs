fn main() {
    use systemconfiguration::schema_definitions;

    println!(
        "DNS domain name key: {}",
        schema_definitions::SC_PROP_NET_DNS_DOMAIN_NAME
    );
    println!(
        "IPv4 config method key: {}",
        schema_definitions::SC_PROP_NET_IPV4_CONFIG_METHOD
    );
    println!(
        "DHCP value: {}",
        schema_definitions::SC_VAL_NET_IPV4_CONFIG_METHOD_DHCP
    );
    println!("Status OK: {}", schema_definitions::SC_STATUS_OK);
    println!("Bond status OK: {}", schema_definitions::SC_BOND_STATUS_OK);
}
