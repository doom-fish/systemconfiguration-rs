use systemconfiguration::{NetworkInterface, Reachability};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== SystemConfiguration interfaces ==");
    for interface in NetworkInterface::copy_all()? {
        println!(
            "bsd={:?} type={:?} mac={:?}",
            interface.bsd_name()?,
            interface.interface_type()?,
            interface.hardware_address_string()?
        );
    }

    let reachability = Reachability::with_name("apple.com")?;
    println!("apple.com flags: {}", reachability.flags()?);
    println!("✅ systemconfiguration interfaces + reachability OK");
    Ok(())
}
