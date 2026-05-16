use systemconfiguration::Schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = Schema::catalog()?;
    println!(
        "components={} entities={} proxies={}",
        catalog.components.len(),
        catalog.entities.len(),
        catalog.proxies.len()
    );
    Ok(())
}
