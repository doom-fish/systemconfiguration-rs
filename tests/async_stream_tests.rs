//! Integration tests for the `async_api` stream module.

#[cfg(feature = "async")]
mod async_stream {
    use systemconfiguration::async_api::{
        DynamicStoreNotificationStream, PreferencesNotificationStream, ReachabilityStream,
    };

    #[test]
    fn dynamic_store_subscribe_and_drop() {
        let stream = DynamicStoreNotificationStream::subscribe(
            "test-async-ds",
            &["State:/Network/Global/IPv4"],
            &[],
            8,
        )
        .expect("DynamicStoreNotificationStream::subscribe should succeed");
        let _next = stream.next();
        assert_eq!(stream.buffered_count(), 0);
        assert!(stream.try_next().is_none());
    }

    #[test]
    fn dynamic_store_subscribe_patterns_only() {
        let stream = DynamicStoreNotificationStream::subscribe(
            "test-async-ds-patterns",
            &[],
            &["State:/Network/Interface/.*/IPv4"],
            8,
        )
        .expect("subscribe with patterns should succeed");
        assert_eq!(stream.buffered_count(), 0);
        assert!(stream.try_next().is_none());
    }

    #[test]
    fn reachability_subscribe_and_drop() {
        let stream = ReachabilityStream::subscribe("localhost", 8)
            .expect("ReachabilityStream::subscribe should succeed");
        let _next = stream.next();
        assert!(stream.buffered_count() <= 8);
        while let Some(event) = stream.try_next() {
            assert!(event.flags.is_reachable());
        }
    }

    #[test]
    fn reachability_stream_delivers_and_tears_down_repeatedly() {
        let mut received = 0;
        for _ in 0..10 {
            let stream = ReachabilityStream::subscribe("localhost", 8)
                .expect("ReachabilityStream::subscribe should succeed");
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
            while std::time::Instant::now() < deadline {
                if let Some(event) = stream.try_next() {
                    assert!(event.flags.is_reachable());
                    received += 1;
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            drop(stream);
        }
        assert!(received > 0);
    }

    #[test]
    fn preferences_subscribe_and_drop() {
        let stream = PreferencesNotificationStream::subscribe("test-async-prefs", 8)
            .expect("PreferencesNotificationStream::subscribe should succeed");
        let _next = stream.next();
        assert_eq!(stream.buffered_count(), 0);
        assert!(stream.try_next().is_none());
    }

    #[test]
    fn multiple_streams_coexist() {
        let dynamic_store = DynamicStoreNotificationStream::subscribe(
            "test-coexist-ds",
            &["State:/Network/Global/IPv4"],
            &[],
            4,
        )
        .expect("dynamic store subscribe");
        let reachability =
            ReachabilityStream::subscribe("8.8.8.8", 4).expect("reachability subscribe");
        let preferences = PreferencesNotificationStream::subscribe("test-coexist-prefs", 4)
            .expect("preferences subscribe");
        assert!(dynamic_store.try_next().is_none());
        assert!(reachability.try_next().is_none());
        assert!(preferences.try_next().is_none());
    }
}
