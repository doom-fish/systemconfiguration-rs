use systemconfiguration::Schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = Schema::catalog()?;
    println!(
        "all={} components={} entities={} proxies={} setup_domain={}",
        catalog.all.len(),
        catalog.components.len(),
        catalog.entities.len(),
        catalog.proxies.len(),
        catalog.get("kSCDynamicStoreDomainSetup").unwrap_or_default()
    );
    Ok(())
}
