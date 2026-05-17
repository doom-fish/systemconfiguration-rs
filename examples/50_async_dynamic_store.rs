//! Example: subscribe to `DynamicStore` key-change notifications as an async stream.
//! Exits immediately without waiting for events (headless-safe).

#[cfg(feature = "async")]
fn main() {
    let stream = match systemconfiguration::async_api::DynamicStoreNotificationStream::subscribe(
        "async-ds-example",
        &["State:/Network/Global/IPv4"],
        &[],
        16,
    ) {
        Ok(stream) => stream,
        Err(error) => {
            eprintln!("subscribe failed (may require entitlements): {error}");
            return;
        }
    };

    let buffered = stream.buffered_count();
    let event = stream.try_next();
    println!("DynamicStoreNotificationStream OK — buffered={buffered}, immediate_event={event:?}");
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("enable the async feature: cargo run --example 50_async_dynamic_store --features async");
}
