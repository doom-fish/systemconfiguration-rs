mod common;

use systemconfiguration::DynamicStore;

#[test]
fn dynamic_store_reads_existing_state_and_key_helpers() -> Result<(), Box<dyn std::error::Error>> {
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
    let _ = store.dhcp_info(None)?;

    assert!(!DynamicStore::computer_name_key()?.is_empty());
    assert!(!DynamicStore::console_user_key()?.is_empty());
    Ok(())
}
