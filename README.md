# systemconfiguration-rs

Safe Rust bindings for Apple’s `SystemConfiguration.framework` on macOS.

`systemconfiguration-rs` focuses on the APIs most useful for network tooling and
system inspection:

- `Reachability` wrappers for hostname / socket-address reachability checks
- `DynamicStore` wrappers for configd key lookup, mutation, and notifications
- `NetworkInterface` enumeration (`BSDName`, interface type, display name, MAC)
- `Preferences` + `NetworkService` wrappers for system preference sessions and
  service inspection

## Status

Initial `0.1.0` coverage targets the public C APIs that are practical for
listing interfaces, checking reachability, enumerating services, and reading or
updating dynamic-store / preferences values.

## Installation

```toml
[dependencies]
systemconfiguration-rs = "0.1"
```

## Quick start

```rust
use systemconfiguration::{NetworkInterface, Reachability};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for interface in NetworkInterface::copy_all()? {
        println!(
            "{:?} {:?}",
            interface.bsd_name()?,
            interface.interface_type()?
        );
    }

    let reachability = Reachability::with_name("apple.com")?;
    println!("apple.com => {}", reachability.flags()?);
    Ok(())
}
```

## Highlights

- Pure Rust + C FFI (no Swift bridge required)
- `Reachability::with_name`, `Reachability::with_address`, `flags`,
  `set_callback`, and `schedule_with_run_loop_current`
- `DynamicStore::copy_value`, `set_value`, `copy_key_list`, and
  `set_notification_keys`
- `NetworkInterface::copy_all` + per-interface metadata accessors
- `Preferences::new`, `lock`, `set_value`, `commit_changes`, and
  `network_services`

## API notes

- `PropertyList` currently provides ergonomic constructors for common string and
  boolean values plus `description()` for debugging arbitrary CoreFoundation
  payloads.
- Writing preferences or dynamic-store values can require elevated privileges or
  a properly scoped preferences session.
- `SCNetworkReachability*` is deprecated by Apple in favor of Network.framework,
  but it remains widely used and is exposed here as requested.

## Smoke example

```bash
cargo run --example 01_interfaces_and_reachability
```

Expected tail output:

```text
✅ systemconfiguration interfaces + reachability OK
```

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
