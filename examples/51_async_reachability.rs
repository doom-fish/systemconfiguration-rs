//! Example: subscribe to reachability changes for `apple.com` as an async stream.

#[cfg(feature = "async")]
fn main() {
    let stream = match systemconfiguration::async_api::ReachabilityStream::subscribe("apple.com", 8)
    {
        Ok(stream) => stream,
        Err(error) => {
            eprintln!("subscribe failed: {error}");
            return;
        }
    };

    let buffered = stream.buffered_count();
    let event = stream.try_next();
    println!("ReachabilityStream OK — buffered={buffered}, immediate_event={event:?}");
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!(
        "enable the async feature: cargo run --example 51_async_reachability --features async"
    );
}
