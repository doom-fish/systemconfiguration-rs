use systemconfiguration::CaptiveNetwork;

#[test]
fn captive_network_supported_interfaces_is_safe_to_query() {
    let _ = CaptiveNetwork::supported_interfaces();
}
