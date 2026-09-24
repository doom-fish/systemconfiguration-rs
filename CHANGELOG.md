# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2026-09-24

### Security

- `DynamicStore`, `Preferences` and `NetworkConnection` handed their callback
  state to SystemConfiguration without retain/release, and dropping the last
  handle neither unscheduled the object nor cleared its callback. A scheduled
  run-loop source or dispatch queue keeps the SystemConfiguration object alive,
  so it went on calling into freed memory (a use-after-free from safe code).
  SystemConfiguration now holds a reference to each callback for as long as it
  can call it, and dropping the last handle unschedules the object from every
  run loop and mode, clears its dispatch queue and callback, and invalidates the
  store's run-loop sources.
- `Reachability` callbacks registered with `set_callback_send` on a dispatch
  queue, and the async `ReachabilityStream` and `PreferencesNotificationStream`,
  could free their callback state while a callout was still running, because
  SystemConfiguration runs the callout after releasing its own lock.
- `Reachability::set_callback` handed SystemConfiguration an unretained pointer
  to its closure, so a closure that dropped or replaced its own `Reachability`
  from inside the callback freed itself while it was running (a use-after-free
  from safe code). Every `Reachability` callback now lives in a retained
  context that SystemConfiguration releases after its last callout.

### Fixed

- Replacing a `Preferences` callback while it runs on a dispatch queue no longer
  frees the running closure.
- Dropping one clone of a `DynamicStore`, `Preferences` or `NetworkConnection`
  no longer frees the callback the other clones still use.
- A `DynamicStore` with a run-loop source that was never scheduled is freed
  instead of leaking together with its configd session.
- Scheduling is no longer limited to the current thread's run loop in the
  default mode or a queue private to the crate.
- `build.rs` no longer adds the toolchain's Swift 5.5 back-deployment
  directory (`usr/lib/swift-5.5/macosx`) to the link search path or the
  rpath. Its old `libswift_Concurrency.dylib` shadowed the SDK's
  `libswift_Concurrency.tbd` in every binary that depends on this crate, so
  linking failed next to a Swift bridge that uses newer concurrency APIs,
  such as apple-localauthentication's.

### Changed

- **Breaking:** `Preferences::lock` returns a `PreferencesLock` that unlocks
  when dropped; `PreferencesLock::unlock` reports the result.
- **Breaking:** `Reachability::set_callback` takes a `Send` closure and
  replaces `set_callback_send`. It works with any run loop or dispatch queue,
  and the callback may drop or replace its own registration.
- **Breaking:** `DynamicStoreRunLoopSource::schedule` and
  `Preferences::schedule_with_run_loop` return `kSCStatusInvalidArgument` for a
  run loop other than the calling thread's or the main thread's.
  SystemConfiguration runs the schedule and cancel callouts of the run-loop
  source behind both without locking, and Core Foundation cancels a source on
  the thread that exits or frees its run loop, which raced with the owning
  thread once `CFRunLoop` became `Send`. `Reachability` and `NetworkConnection`
  accept any thread's run loop.
- **Breaking:** the fixed-target helpers are removed:
  `schedule_with_run_loop_current`, `unschedule_from_run_loop_current` and
  `set_dispatch_queue_global` on `Reachability`, `Preferences` and
  `NetworkConnection`, `DynamicStore::set_dispatch_queue_global`, and
  `DynamicStoreRunLoopSource::{schedule_current_default_mode,
  unschedule_current_default_mode}`. Pass `&CFRunLoop::current()` and
  `RunLoopMode::Default`, or a `DispatchQueue`, to the general methods; the
  `_global` helpers used a queue private to the crate, not a global queue.
- `Preferences::set_callback` and `clear_callback` take `&self`.
- Clones of `DynamicStore`, `Preferences` and `NetworkConnection` share one
  registration, which lives until the last clone is dropped.
- A callback that is not running when its handle is dropped is dropped at once,
  even when SystemConfiguration keeps its own object alive (it never frees an
  `SCNetworkConnection` that has a callback).
- `doom-fish-utils` is a regular dependency; the `async` feature only enables
  the stream module. `apple-cf` now enables its `dispatch` feature.
- `rust-version` is 1.82 (was 1.76). Requires `apple-cf >=0.11, <0.12` and
  `doom-fish-utils >=0.4.1, <0.5`.

### Added

- `RunLoopMode` (`Default`, `Common`, `Named`) and scheduling on any `CFRunLoop`:
  `DynamicStoreRunLoopSource::{schedule, unschedule, is_valid}`,
  `Preferences::{schedule_with_run_loop, unschedule_from_run_loop}`,
  `NetworkConnection::{schedule_with_run_loop, unschedule_from_run_loop}` and
  `Reachability::{schedule_with_run_loop, unschedule_from_run_loop}`.
- `set_dispatch_queue(&DispatchQueue)` on `DynamicStore`, `Preferences`,
  `NetworkConnection` and `Reachability`.
- Re-exports of `CFRunLoop`, `DispatchQueue` and `DispatchQoS` from `apple-cf`.

### Removed

- `Preferences::unlock`; use the `PreferencesLock` guard.
- `Reachability::set_callback_send`; `set_callback` takes `Send` closures.
- The `*_run_loop_current`, `*_current_default_mode` and
  `set_dispatch_queue_global` helpers (see Changed).

## [0.5.6] - 2026-06-06

- User callbacks for reachability, preferences and network connections are
  wrapped in panic guards, and the unused Swift bridge C header is gone.

## [0.5.5] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.5.4] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.5.3] - 2026-05-18

### Changed

- Added one-line rustdoc coverage across the safe wrapper surface and `schema_definitions`, bringing `cargo +nightly rustdoc --lib --all-features -- -Z unstable-options --show-coverage` to 77.0% documented items.
- Bumped the crate version to `0.5.3`.

## [0.5.2] - 2026-05-18

### Changed

- Re-exported `Boolean` from `apple_cf::raw` in `raw_ffi`, removing the remaining crate-local primitive alias.

## [0.5.1] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.5.0] - 2026-05-18

### Changed

- Re-exported the raw `CFAllocatorRef`, `CFArrayRef`, `CFDataRef`, `CFDateRef`,
  `CFDictionaryRef`, `CFPropertyListRef`, `CFRunLoopRef`, and `CFStringRef`
  typedefs from `apple-cf` in `raw_ffi`, removing the crate-local duplicates.
- Added `apple-cf` (`>=0.8, <0.9`) with `default-features = false` and the
  `cg` feature enabled for shared CoreFoundation/CoreGraphics type coverage.
- Bumped the crate version to `0.5.0`.

## [0.4.1] - 2026-05-18

### Fixed

- **Thread join in `DynamicStoreNotificationStream`**: `sc_dynamic_store_notification_unsubscribe`
  now waits for the run-loop thread to exit (`threadDone.wait()`) before returning.
  Previously, `CFRunLoopStop`+`CFRunLoopWakeUp` were called but the thread was
  never joined, creating a window where the thread could access the Rust sender
  box after `Drop` freed it.  A `DispatchSemaphore` (`threadDone`) is now
  signalled by the thread closure immediately after `CFRunLoopRun()` returns.
- **Panic safety in `extern "C"` callbacks**: all three `async_api` callbacks
  (`dynamic_store_async_cb`, `reachability_async_cb`, `preferences_async_cb`)
  now wrap user-code invocations in
  `doom_fish_utils::panic_safe::catch_user_panic`.  The synchronous
  `dynamic_store_callback` in `dynamic_store.rs` now wraps user closures in
  `std::panic::catch_unwind`.  Unhandled panics across the Swift/C FFI boundary
  are undefined behaviour.
- **SAFETY comments** added to the three `ctx`-pointer dereferences in `async_api.rs`
  callbacks and to the `info`-pointer dereference in `dynamic_store_callback`.
- **`unsafe impl Send/Sync for SubscriptionHandle`** now carries doc comments
  explaining the single-use / no-alias ownership invariants that make them sound.



### Added

- `async` feature with an `async_api` module exposing three
  `BoundedAsyncStream`-based notification streams:
  - `DynamicStoreNotificationStream` — wraps `SCDynamicStore` key-change
    notifications via a dedicated `CFRunLoop` thread.
  - `ReachabilityStream` — wraps `SCNetworkReachability` callbacks via a
    private dispatch queue.
  - `PreferencesNotificationStream` — wraps `SCPreferences` notifications via a
    private dispatch queue.
- Swift bridge entry points in `AsyncStream.swift` for async subscribe /
  unsubscribe lifecycle management.
- Async examples `50_async_dynamic_store`, `51_async_reachability`, and
  `52_async_preferences`.
- Async smoke coverage in `tests/async_stream_tests.rs`.

### Changed

- Bumped the crate version to `0.4.0`.

## [0.3.0] - 2026-05-17

### Added

- Public `schema_definitions` module with compile-time SystemConfiguration schema,
  interface, protocol, status, bond, reachability, captive-network, and
  connection constants exposed as Rust `pub const`s.
- `examples/23_schema_definitions_constants.rs` and
  `tests/schema_definitions_tests.rs` smoke coverage for the new constants.

### Changed

- Reclassified every stale entry in `COVERAGE_AUDIT_V2.md`, closing all 475 v2
  audit gaps and bringing the v2 audit to 582 verified symbols with 13
  documented exemptions.
- Verified the existing safe-wrapper coverage for `BondInterface`,
  `VlanInterface`, `NetworkConnection`, `DynamicStore`, `NetworkReachability`,
  `NetworkService`, `NetworkSet`, `Preferences`, `NetworkInterface`,
  `NetworkProtocol`, and `SystemConfiguration`.
- Bumped the crate version to `0.3.0`.

## [0.2.2] - 2026-05-17

### Added

- `DynamicStore::new_with_options` and
  `DynamicStore::new_with_options_and_callback` for the remaining generic
  `SCDynamicStoreCreateWithOptions` coverage.
- `Reachability::type_id`, `Reachability::set_callback_send`,
  `Reachability::set_dispatch_queue_global`, and
  `Reachability::clear_dispatch_queue` to close the last reachability gaps.

### Changed

- Closed the remaining safe-wrapper partials in `COVERAGE.md`, bringing the
  non-skipped safe audit to 100%.
- Bumped the crate version to `0.2.2`.

## [0.2.1] - 2026-05-16

### Added

- `NetworkConnection`, `BondInterface`, `BondStatus`, `VlanInterface`, and
  `SystemConfiguration` safe wrappers for the remaining public macOS families in
  the audited headers.
- Advanced `DynamicStore` helpers for callbacks, run-loop sources, dispatch
  queues, bulk updates, DHCP accessors, and formatted key creation.
- Advanced `Preferences` helpers for authorization, callbacks, run-loop
  scheduling, dispatch queues, and type IDs.
- Full `SCSchemaDefinitions.h` exposure through `SchemaCatalog::all` plus
  symbol-name lookup helpers.

### Changed

- Closed the final 259 non-exempt coverage gaps in `COVERAGE_AUDIT.md`, taking
  the audited bridge coverage to 100.00%.
- Expanded smoke tests and the schema example to validate the new coverage.

## [0.2.0] - 2026-05-16

### Changed

- Replaced the old direct-first C FFI architecture with a SwiftPM-built bridge
  patterned after `screencapturekit-rs`.
- Moved the primary macOS implementation behind per-area Swift bridge files plus
  matching safe Rust modules.
- Kept direct C symbols available behind the `raw-ffi` Cargo feature.

### Added

- Safe wrappers for the requested logical areas: `DynamicStore`,
  `NetworkConfiguration`, `Reachability`, `Preferences`, `Schema`,
  `NetworkService`, `NetworkSet`, `NetworkInterface`, `NetworkProtocol`,
  `ConsoleUser`, and `CaptiveNetwork`.
- 11 numbered examples covering each logical area.
- 11 per-area smoke test files and shared test helpers.
- `COVERAGE.md` with a per-header audit of implemented, partial, and skipped
  APIs.

## [0.1.0] - 2026-05-16

### Added

- `Reachability` wrappers for `SCNetworkReachabilityCreateWithName`,
  `SCNetworkReachabilityCreateWithAddress`, `GetFlags`, `SetCallback`, and
  `ScheduleWithRunLoop`.
- `DynamicStore` wrappers for `SCDynamicStoreCreate`, `CopyValue`, `SetValue`,
  `CopyKeyList`, and `SetNotificationKeys`.
- `NetworkInterface` wrappers for `SCNetworkInterfaceCopyAll` and key metadata
  accessors (`BSDName`, interface type, display name, hardware address).
- `Preferences` and `NetworkService` wrappers for `SCPreferencesCreate`,
  `Lock`, `SetValue`, `CommitChanges`, `SCNetworkServiceCopyAll`,
  `SCNetworkServiceGetServiceID`, and `SCNetworkServiceGetEnabled`.
- `examples/01_interfaces_and_reachability.rs` smoke example covering network
  interface enumeration and an `apple.com` reachability query.
