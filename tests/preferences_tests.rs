use systemconfiguration::{Preferences, PreferencesNotification};

#[test]
fn preferences_support_read_only_queries_and_paths() -> Result<(), Box<dyn std::error::Error>> {
    assert!(Preferences::type_id() > 0);

    let prefs = Preferences::new("systemconfiguration-rs.preferences-tests", None)?;
    let keys = prefs.copy_key_list();
    assert!(!keys.is_empty());

    let first_key = &keys[0];
    let _ = prefs.get_value(first_key)?;
    let _ = prefs.path_get_value("/")?;
    let _ = prefs.path_get_link("/")?;
    assert!(prefs.signature().is_some());

    let auth_prefs = Preferences::new_with_authorization("systemconfiguration-rs.preferences-auth-tests", None)?;
    assert!(!auth_prefs.copy_key_list().is_empty());

    let notifications = PreferencesNotification::COMMIT | PreferencesNotification::APPLY;
    assert!(notifications.contains(PreferencesNotification::COMMIT));

    let mut callback_prefs = Preferences::new_with_callback(
        "systemconfiguration-rs.preferences-callback-tests",
        None,
        |_| {},
    )?;
    callback_prefs.schedule_with_run_loop_current()?;
    callback_prefs.unschedule_from_run_loop_current()?;
    callback_prefs.set_dispatch_queue_global()?;
    callback_prefs.clear_dispatch_queue()?;
    callback_prefs.clear_callback()?;

    Ok(())
}
