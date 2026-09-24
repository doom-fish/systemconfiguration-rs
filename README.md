# systemconfiguration-rs

Safe Rust bindings for Apple’s `SystemConfiguration.framework` on macOS.

Cargo builds a small SwiftPM static library, Rust owns opaque retained
handles, and the public API stays ergonomic on the Rust side.
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

## Requirements

- macOS 10.15 or later (the Swift bridge's deployment target)
- Xcode or the Command Line Tools with a Swift toolchain
- Root, or an `AuthorizationRef` for `Preferences`, to change configuration:
  without it, SystemConfiguration answers `kSCStatusAccessError` (code 1003)
  to dynamic-store writes and notifications and to preferences commits and
  locks. Reading, watching keys and reachability need no privileges.

## Installation

```toml
[dependencies]
systemconfiguration-rs = "0.6"
```

Enable async notification streams when needed:

```toml
[dependencies]
systemconfiguration-rs = { version = "0.6", features = ["async"] }
```

Enable raw C access when needed:

```toml
[dependencies]
systemconfiguration-rs = { version = "0.6", features = ["raw-ffi"] }
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

## Callbacks and scheduling

`DynamicStore`, `Preferences`, `NetworkConnection` and `Reachability` deliver
callbacks on a `CFRunLoop` in a `RunLoopMode` (`Default`, `Common` or
`Named`), or on a `DispatchQueue` you pass in. `CFRunLoop`, `DispatchQueue`
and `DispatchQoS` are re-exported from `apple-cf`. `CFRunLoop` is `Send`, so a
worker thread can hand its `CFRunLoop::current()` to the code that schedules.

```rust,no_run
use systemconfiguration::{DispatchQoS, DispatchQueue, DynamicStore};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = DynamicStore::new_with_callback("com.example.watcher", |keys| {
        println!("changed: {keys:?}");
    })?;
    let key = DynamicStore::computer_name_key()?;
    store.set_notification_keys(&[key.as_str()], &[] as &[&str])?;
    let queue = DispatchQueue::new("com.example.watcher", DispatchQoS::Utility);
    store.set_dispatch_queue(&queue)?;
    Ok(())
}
```

- `Reachability` and `NetworkConnection` can be scheduled on any thread's run
  loop. A `DynamicStore` run-loop source and `Preferences` only accept the
  calling thread's run loop or the main run loop, and return
  `kSCStatusInvalidArgument` otherwise: SystemConfiguration runs the schedule
  and cancel callouts of the run-loop source behind them without locking, and
  Core Foundation cancels a source on the thread that exits or frees its run
  loop. Use a `DispatchQueue` to receive their callbacks on another thread.
- Callbacks are `Send` closures, because SystemConfiguration may run them on
  another thread and release them there.
- SystemConfiguration holds its own reference to the callback for as long as
  it can call it, so a callback that is already running when you drop the
  handle or replace the callback, even from inside the callback, finishes
  safely.
- Dropping the last handle (clones share one registration) stops new
  callbacks, then unschedules the object from every run loop and mode it was
  scheduled on, clears its dispatch queue and callback, and invalidates the
  `DynamicStore`'s run-loop sources. The closure is dropped at that point
  unless it is running.
- `Preferences::lock` returns a `PreferencesLock` that unlocks when dropped.
- SystemConfiguration never frees an `SCNetworkConnection` that has a
  callback; the Rust closure is still dropped with the last handle.

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
- 15 numbered examples under `examples/`, 15 integration test files under
  `tests/`, and unit tests for callback teardown

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

- Writing to the dynamic store or preferences needs root (see Requirements);
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
