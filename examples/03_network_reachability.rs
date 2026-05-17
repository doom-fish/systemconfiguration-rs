use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use systemconfiguration::Reachability;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let by_name = Reachability::with_name("localhost")?;
    println!("localhost => {}", by_name.flags()?);

    let by_address =
        Reachability::with_address(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)))?;
    println!("127.0.0.1 => {}", by_address.flags()?);
    Ok(())
}
