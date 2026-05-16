use systemconfiguration::Preferences;

#[test]
fn preferences_support_read_only_queries_and_paths() -> Result<(), Box<dyn std::error::Error>> {
    let prefs = Preferences::new("systemconfiguration-rs.preferences-tests", None)?;
    let keys = prefs.copy_key_list();
    assert!(!keys.is_empty());

    let first_key = &keys[0];
    let _ = prefs.get_value(first_key)?;
    let _ = prefs.path_get_value("/")?;
    let _ = prefs.path_get_link("/")?;
    assert!(prefs.signature().is_some());
    Ok(())
}
