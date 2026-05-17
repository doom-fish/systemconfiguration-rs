use systemconfiguration::{NetworkSet, Preferences};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prefs = Preferences::new("systemconfiguration-rs.network-sets-example", None)?;
    let current = NetworkSet::copy_current(&prefs).and_then(|set| set.name());
    println!("current_set={current:?}");
    for set in NetworkSet::copy_all(&prefs) {
        println!(
            "set_id={:?} name={:?} services={}",
            set.set_id(),
            set.name(),
            set.copy_services().len()
        );
    }
    Ok(())
}
