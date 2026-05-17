use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use systemconfiguration::Reachability;

#[test]
fn reachability_supports_name_address_and_callback_operations(
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(Reachability::type_id() > 0);

    let mut by_name = Reachability::with_name("localhost")?;
    let flags = by_name.flags()?;
    let _ = flags.is_reachable();

    by_name.set_callback(|_| {})?;
    by_name.clear_callback()?;
    by_name.set_callback_send(|_| {})?;
    by_name.set_dispatch_queue_global()?;
    by_name.clear_dispatch_queue()?;
    by_name.clear_callback()?;

    let mut by_address =
        Reachability::with_address(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)))?;
    let _ = by_address.flags()?;
    by_address.schedule_with_run_loop_current()?;
    by_address.unschedule_from_run_loop_current()?;

    let pair = Reachability::with_address_pair(
        None,
        Some(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))),
    )?;
    let _ = pair.flags()?;
    Ok(())
}
