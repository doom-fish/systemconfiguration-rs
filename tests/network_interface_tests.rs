use systemconfiguration::NetworkInterface;

#[test]
fn network_interfaces_expose_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let interfaces = NetworkInterface::copy_all()?;
    if let Some(interface) = interfaces.first() {
        let _ = interface.bsd_name()?;
        let _ = interface.interface_type()?;
        let _ = interface.localized_display_name()?;
        let _ = interface.hardware_address_string()?;
        let _ = interface.configuration();
        let _ = interface.supported_interface_types();
        let _ = interface.supported_protocol_types();
        let _ = interface.mtu_info()?;
    }
    Ok(())
}
