use systemconfiguration::Preferences;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prefs = Preferences::new("systemconfiguration-rs.preferences-example", None)?;
    let keys = prefs.copy_key_list();
    println!("key_count={}", keys.len());
    if let Some(first_key) = keys.first() {
        println!("first_key={first_key}");
        println!("first_value_present={}", prefs.get_value(first_key)?.is_some());
    }
    println!("signature_present={}", prefs.signature().is_some());
    Ok(())
}
