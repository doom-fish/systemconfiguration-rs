use systemconfiguration::DynamicStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = DynamicStore::new_with_session_keys("systemconfiguration-rs.dynamic-store-example")?;
    println!("computer_name={:?}", store.computer_name());
    println!("local_host_name={:?}", store.local_host_name());
    println!("location={:?}", store.location());

    let global_ipv4 = DynamicStore::network_global_entity_key("State", "IPv4")?;
    println!(
        "global_ipv4={:?}",
        store.copy_value(&global_ipv4)?.map(|value| value.description()).transpose()?
    );
    println!("proxies_present={}", store.proxies().is_some());
    Ok(())
}
