# systemconfiguration-rs

Safe Rust bindings for Apple’s `SystemConfiguration.framework` on macOS.

Version `0.4.0` continues the `screencapturekit-rs`-style Swift bridge
introduced in `0.2.0`: Cargo builds a small SwiftPM static library, Rust owns
opaque retained handles, and the public API stays ergonomic on the Rust side.
If you need async notification streams, enable the `async` feature. If you
still need low-level C symbols, enable the `raw-ffi` feature.

## Covered areas

`systemconfiguration-rs` now ships safe wrappers for the logical areas covered
by the current bridge release:

- `DynamicStore`
- `NetworkConfiguration` overview helpers
- `NetworkConnection`
- `Reachability` / `NetworkReachability`
- `Preferences`
- `Schema`
- `NetworkService`
- `NetworkSet`
- `NetworkInterface`
- `BondInterface`
- `VlanInterface`
- `NetworkProtocol`
- `ConsoleUser`
- `CaptiveNetwork`
- `SystemConfiguration` error helpers
- Optional `async_api` notification streams (`features = ["async"]`)

See [COVERAGE.md](COVERAGE.md) for the per-header audit, including the APIs that
are intentionally skipped on modern macOS.

## Installation

```toml
[dependencies]
systemconfiguration-rs = "0.4"
```

Enable async notification streams when needed:

```toml
[dependencies]
systemconfiguration-rs = { version = "0.4", features = ["async"] }
```

Enable raw C access when needed:

```toml
[dependencies]
systemconfiguration-rs = { version = "0.4", features = ["raw-ffi"] }
```

The crate name is `systemconfiguration-rs`; the Rust library name is
`systemconfiguration`.

## Quick start

```rust,no_run
use systemconfiguration::{DynamicStore, NetworkConfiguration, Reachability};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = DynamicStore::new("com.example.systemconfiguration-rs")?;
    println!("computer_name={:?}", store.computer_name());

    let reachability = Reachability::with_name("apple.com")?;
    println!("apple.com => {}", reachability.flags()?);

    let overview = NetworkConfiguration::overview()?;
    println!(
        "interfaces={} services={} current_set={:?}",
        overview.interface_count,
        overview.service_count,
        overview.current_set_name
    );

    Ok(())
}
```

## Highlights

- Swift bridge primary implementation with one Swift file per logical area
- Safe Rust wrappers for property lists, preferences sessions, dynamic-store
  callbacks, services, sets, interfaces, bond/VLAN configuration, network
  connections, reachability, captive-network helpers, console-user lookup, and
  SystemConfiguration error helpers
- Optional `async` feature exposing `DynamicStoreNotificationStream`,
  `ReachabilityStream`, and `PreferencesNotificationStream` backed by
  `doom-fish-utils::stream::BoundedAsyncStream`
- `raw-ffi` feature preserving direct access to the underlying C APIs already
  declared by the crate
- 14 numbered examples under `examples/` and 15 smoke tests under `tests/`

## Architecture

- `build.rs` builds `swift-bridge/Package.swift` into a static library and links
  it into the Rust crate.
- Swift bridge entry points are `@_cdecl` functions returning C primitives or
  opaque retained pointers.
- Rust owns those handles via `OwnedHandle` in `src/bridge.rs` and releases them
  through shared bridge retain/release functions.
- Some structured results are serialized as JSON in Swift and decoded with
  `serde` on the Rust side.

## Examples

Run individual examples as needed:

- `cargo run --example 01_dynamic_store_overview`
- `cargo run --example 03_network_reachability`
- `cargo run --example 06_network_services`
- `cargo run --example 11_captive_network`
- `cargo run --example 50_async_dynamic_store --features async`
- `cargo run --example 51_async_reachability --features async`
- `cargo run --example 52_async_preferences --features async`

## API notes

- Writing to the dynamic store or preferences can require elevated privileges;
  read-only smoke examples are used where the host environment denies mutation.
- Apple deprecates `SCNetworkReachability*` in favor of `Network.framework`, but
  these APIs remain wrapped because they are still widely deployed.
- Async notification streams own their subscription handles; drop the stream to
  unsubscribe and tear down the underlying run loop or dispatch queue.
- `CaptiveNetwork` does not wrap `CNCopyCurrentNetworkInfo`, because that API is
  unavailable on modern macOS and generally entitlement-gated.
- `Schema::catalog()` now exposes the full `SCSchemaDefinitions.h` catalog in
  `SchemaCatalog::all`, while keeping the original curated convenience groups.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
