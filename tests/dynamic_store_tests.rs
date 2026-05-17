mod common;

use systemconfiguration::{DynamicStore, PropertyList};

#[test]
fn dynamic_store_reads_existing_state_and_key_helpers() -> Result<(), Box<dyn std::error::Error>> {
    assert!(DynamicStore::type_id() > 0);

    let store = DynamicStore::new_with_session_keys("systemconfiguration-rs.dynamic-store-tests")?;
    let global_ipv4 = DynamicStore::network_global_entity_key("State", "IPv4")?;
    let _ = store.copy_value(&global_ipv4)?;

    let keys = store.copy_key_list("State:/Network/.*")?;
    if let Some(key) = keys.first() {
        let _ = store.copy_value(key)?;
    }

    let _ = store.computer_name();
    let _ = store.local_host_name();
    let _ = store.location();
    let _ = store.proxies();
    if let Some(info) = store.dhcp_info(None)? {
        let _ = DynamicStore::dhcp_option_data(&info, 1);
        let _ = DynamicStore::dhcp_lease_start_time(&info);
        let _ = DynamicStore::dhcp_lease_expiration_time(&info);
    }

    let options = PropertyList::from_json("{}")?;
    let _ = DynamicStore::new_with_options(
        "systemconfiguration-rs.dynamic-store-options-tests",
        &options,
    )?;

    let callback_store = DynamicStore::new_with_options_and_callback(
        "systemconfiguration-rs.dynamic-store-callback-tests",
        &options,
        |_| {},
    )?;
    let computer_name_key = DynamicStore::computer_name_key()?;
    callback_store.set_notification_keys(&[computer_name_key.as_str()], &[] as &[&str])?;
    let source = callback_store.create_run_loop_source(0)?;
    source.schedule_current_default_mode()?;
    source.unschedule_current_default_mode()?;
    callback_store.set_dispatch_queue_global()?;
    callback_store.clear_dispatch_queue()?;

    let custom_key = DynamicStore::key_create("%@/%@", &["State:/Network", "Global/IPv4"])?;
    assert_eq!(custom_key, "State:/Network/Global/IPv4");
    assert!(!DynamicStore::computer_name_key()?.is_empty());
    assert!(!DynamicStore::console_user_key()?.is_empty());
    Ok(())
}
