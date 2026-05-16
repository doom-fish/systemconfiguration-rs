use systemconfiguration::NetworkInterface;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for interface in NetworkInterface::copy_all()? {
        println!(
            "bsd={:?} type={:?} name={:?}",
            interface.bsd_name()?,
            interface.interface_type()?,
            interface.localized_display_name()?
        );
    }
    Ok(())
}
