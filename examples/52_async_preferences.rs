//! Example: subscribe to `SCPreferences` change notifications as an async stream.

#[cfg(feature = "async")]
fn main() {
    let stream = match systemconfiguration::async_api::PreferencesNotificationStream::subscribe(
        "async-prefs-example",
        8,
    ) {
        Ok(stream) => stream,
        Err(error) => {
            eprintln!("subscribe failed (may require entitlements): {error}");
            return;
        }
    };

    let buffered = stream.buffered_count();
    let event = stream.try_next();
    println!("PreferencesNotificationStream OK — buffered={buffered}, immediate_event={event:?}");
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!(
        "enable the async feature: cargo run --example 52_async_preferences --features async"
    );
}
