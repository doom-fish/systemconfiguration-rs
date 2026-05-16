use systemconfiguration::{NetworkInterface, NetworkSet, Preferences};

#[test]
fn network_sets_can_be_enumerated() -> Result<(), Box<dyn std::error::Error>> {
    let prefs = Preferences::new("systemconfiguration-rs.network-sets-tests", None)?;
    let sets = NetworkSet::copy_all(&prefs);
    if let Some(set) = sets.first() {
        let _ = set.name();
        let _ = set.set_id();
        let _ = set.service_order();
        let _ = set.copy_services();
        if let Some(interface) = NetworkInterface::copy_all()?.first() {
            let _ = set.contains_interface(interface);
        }
    }
    let _ = NetworkSet::copy_current(&prefs);
    Ok(())
}
