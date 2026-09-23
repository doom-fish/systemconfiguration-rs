mod common;

use std::{
    ffi::c_void,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

use systemconfiguration::{
    CFRunLoop, DispatchQoS, DispatchQueue, DynamicStore, NetworkConnection, Reachability,
    RunLoopMode,
};

fn wait_for(mut condition: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if condition() {
            return true;
        }
        let _ = CFRunLoop::run_in_default_mode(Duration::from_millis(20), false);
    }
    condition()
}

fn witness() -> (Arc<()>, Arc<()>) {
    let witness = Arc::new(());
    let captured = Arc::clone(&witness);
    (witness, captured)
}

struct RunLoopThread {
    run_loop: CFRunLoop,
    stop: mpsc::Sender<()>,
    handle: thread::JoinHandle<()>,
}

impl RunLoopThread {
    fn spawn() -> Self {
        let (run_loop_tx, run_loop_rx) = mpsc::channel();
        let (stop, stop_rx) = mpsc::channel::<()>();
        let handle = thread::spawn(move || {
            run_loop_tx
                .send(CFRunLoop::current().as_ptr() as usize)
                .expect("send run loop");
            while stop_rx.try_recv().is_err() {
                let _ = CFRunLoop::run_in_default_mode(Duration::from_millis(20), false);
            }
        });
        let raw = run_loop_rx.recv().expect("receive run loop") as *mut c_void;
        let run_loop = unsafe { CFRunLoop::from_raw_borrowed(raw) }.expect("run loop");
        Self {
            run_loop,
            stop,
            handle,
        }
    }

    fn join(self) {
        self.stop.send(()).expect("stop run loop thread");
        self.handle.join().expect("join run loop thread");
    }
}

#[test]
fn dynamic_store_keeps_its_callback_until_its_run_loop_source_is_invalidated(
) -> Result<(), Box<dyn std::error::Error>> {
    let (witness, captured) = witness();
    let store = DynamicStore::new_with_callback("systemconfiguration-rs.lifetime-ds", move |_| {
        let _ = &captured;
    })?;
    let key = DynamicStore::computer_name_key()?;
    store.set_notification_keys(&[key.as_str()], &[] as &[&str])?;
    let source = store.create_run_loop_source(0)?;
    let run_loop = CFRunLoop::current();
    source.schedule(&run_loop, RunLoopMode::Default)?;
    source.schedule(&run_loop, RunLoopMode::Common)?;
    source.schedule(
        &run_loop,
        RunLoopMode::Named("systemconfiguration-rs.test-mode"),
    )?;
    source.unschedule(
        &run_loop,
        RunLoopMode::Named("systemconfiguration-rs.test-mode"),
    )?;
    assert!(source.is_valid());
    assert_eq!(Arc::strong_count(&witness), 2);

    drop(store);
    assert!(!source.is_valid());
    assert!(source.schedule(&run_loop, RunLoopMode::Default).is_err());

    drop(source);
    assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    Ok(())
}

#[test]
fn dynamic_store_clones_share_one_registration() -> Result<(), Box<dyn std::error::Error>> {
    let (witness, captured) = witness();
    let store =
        DynamicStore::new_with_callback("systemconfiguration-rs.lifetime-ds-clone", move |_| {
            let _ = &captured;
        })?;
    let key = DynamicStore::computer_name_key()?;
    store.set_notification_keys(&[key.as_str()], &[] as &[&str])?;
    let source = store.create_run_loop_source(0)?;
    source.schedule_current_default_mode()?;

    let clone = store.clone();
    drop(store);
    assert!(source.is_valid());
    assert_eq!(Arc::strong_count(&witness), 2);

    drop(clone);
    assert!(!source.is_valid());
    drop(source);
    assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    Ok(())
}

#[test]
fn dynamic_store_releases_its_callback_when_dropped_on_a_dispatch_queue(
) -> Result<(), Box<dyn std::error::Error>> {
    let (witness, captured) = witness();
    let store =
        DynamicStore::new_with_callback("systemconfiguration-rs.lifetime-ds-queue", move |_| {
            let _ = &captured;
        })?;
    let key = DynamicStore::computer_name_key()?;
    store.set_notification_keys(&[key.as_str()], &[] as &[&str])?;
    let queue = DispatchQueue::new("systemconfiguration-rs.test-ds", DispatchQoS::Utility);
    store.set_dispatch_queue(&queue)?;
    assert_eq!(Arc::strong_count(&witness), 2);

    drop(store);
    assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    Ok(())
}

#[test]
fn dynamic_store_releases_its_callback_when_its_source_was_never_scheduled(
) -> Result<(), Box<dyn std::error::Error>> {
    let (witness, captured) = witness();
    let store =
        DynamicStore::new_with_callback("systemconfiguration-rs.lifetime-ds-idle", move |_| {
            let _ = &captured;
        })?;
    let source = store.create_run_loop_source(0)?;
    drop(store);
    drop(source);
    assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    Ok(())
}

#[test]
fn preferences_replacing_a_callback_releases_the_previous_one(
) -> Result<(), Box<dyn std::error::Error>> {
    let prefs = common::temporary_preferences("lifetime-replace");
    let (first, first_captured) = witness();
    prefs.set_callback(move |_| {
        let _ = &first_captured;
    })?;
    prefs.set_dispatch_queue(&DispatchQueue::new(
        "systemconfiguration-rs.test-prefs",
        DispatchQoS::Utility,
    ))?;
    assert_eq!(Arc::strong_count(&first), 2);

    let (second, second_captured) = witness();
    prefs.set_callback(move |_| {
        let _ = &second_captured;
    })?;
    assert!(wait_for(|| Arc::strong_count(&first) == 1));
    assert_eq!(Arc::strong_count(&second), 2);

    prefs.clear_callback()?;
    assert!(wait_for(|| Arc::strong_count(&second) == 1));
    Ok(())
}

#[test]
fn preferences_drop_unschedules_every_run_loop_and_releases_the_callback(
) -> Result<(), Box<dyn std::error::Error>> {
    let (witness, captured) = witness();
    let prefs = common::temporary_preferences("lifetime-run-loop");
    prefs.set_callback(move |_| {
        let _ = &captured;
    })?;
    let run_loop = CFRunLoop::current();
    prefs.schedule_with_run_loop(&run_loop, RunLoopMode::Default)?;
    prefs.schedule_with_run_loop(
        &run_loop,
        RunLoopMode::Named("systemconfiguration-rs.prefs"),
    )?;
    prefs.schedule_with_run_loop(&CFRunLoop::main(), RunLoopMode::Common)?;

    let clone = prefs.clone();
    drop(prefs);
    assert_eq!(Arc::strong_count(&witness), 2);

    drop(clone);
    assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    Ok(())
}

#[test]
fn preferences_drop_clears_the_dispatch_queue_and_releases_the_callback(
) -> Result<(), Box<dyn std::error::Error>> {
    let (witness, captured) = witness();
    let prefs = common::temporary_preferences("lifetime-queue");
    prefs.set_callback(move |_| {
        let _ = &captured;
    })?;
    prefs.set_dispatch_queue_global()?;
    drop(prefs);
    assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    Ok(())
}

#[test]
fn network_connection_drop_unschedules_and_releases_the_callback(
) -> Result<(), Box<dyn std::error::Error>> {
    let (witness, captured) = witness();
    let connection = NetworkConnection::with_service_id_and_callback(
        "00000000-0000-0000-0000-000000000000",
        move |_| {
            let _ = &captured;
        },
    )?;
    let run_loop = CFRunLoop::current();
    connection.schedule_with_run_loop(&run_loop, RunLoopMode::Default)?;
    connection.schedule_with_run_loop(&run_loop, RunLoopMode::Common)?;
    assert!(connection
        .set_dispatch_queue(&DispatchQueue::new(
            "systemconfiguration-rs.test-connection",
            DispatchQoS::Utility,
        ))
        .is_err());
    assert_eq!(Arc::strong_count(&witness), 2);

    drop(connection);
    assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    Ok(())
}

#[test]
fn reachability_delivers_on_another_threads_run_loop_and_releases_on_drop(
) -> Result<(), Box<dyn std::error::Error>> {
    let worker = RunLoopThread::spawn();
    let hits = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&hits);
    let mut reachability = Reachability::with_name("localhost")?;
    reachability.set_callback_send(move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
    })?;
    reachability.schedule_with_run_loop(&worker.run_loop, RunLoopMode::Default)?;
    assert!(wait_for(|| hits.load(Ordering::SeqCst) > 0));

    drop(reachability);
    assert!(wait_for(|| Arc::strong_count(&hits) == 1));
    worker.join();
    Ok(())
}

#[test]
fn reachability_delivers_on_a_dispatch_queue_and_survives_teardown_races(
) -> Result<(), Box<dyn std::error::Error>> {
    let hits = Arc::new(AtomicUsize::new(0));
    let queue = DispatchQueue::new(
        "systemconfiguration-rs.test-reachability",
        DispatchQoS::Utility,
    );
    for _ in 0..25 {
        let counter = Arc::clone(&hits);
        let mut reachability = Reachability::with_name("localhost")?;
        reachability.set_callback_send(move |_| {
            counter.fetch_add(1, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(1));
        })?;
        reachability.set_dispatch_queue(&queue)?;
        thread::sleep(Duration::from_millis(2));
        drop(reachability);
    }
    assert!(wait_for(|| Arc::strong_count(&hits) == 1));

    let counter = Arc::clone(&hits);
    let before = hits.load(Ordering::SeqCst);
    let mut reachability = Reachability::with_name("localhost")?;
    reachability.set_callback_send(move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
    })?;
    reachability.set_dispatch_queue(&queue)?;
    assert!(wait_for(|| hits.load(Ordering::SeqCst) > before));
    drop(reachability);
    assert!(wait_for(|| Arc::strong_count(&hits) == 1));
    Ok(())
}

#[test]
fn reachability_keeps_non_send_callbacks_on_the_owning_thread(
) -> Result<(), Box<dyn std::error::Error>> {
    let worker = RunLoopThread::spawn();
    let queue = DispatchQueue::new("systemconfiguration-rs.test-local", DispatchQoS::Utility);

    let mut local = Reachability::with_name("localhost")?;
    local.set_callback(|_| {})?;
    assert!(local
        .schedule_with_run_loop(&worker.run_loop, RunLoopMode::Default)
        .is_err());
    assert!(local.set_dispatch_queue(&queue).is_err());
    local.schedule_with_run_loop(&CFRunLoop::current(), RunLoopMode::Default)?;
    local.unschedule_from_run_loop_current()?;

    let mut shared = Reachability::with_name("localhost")?;
    shared.set_callback_send(|_| {})?;
    shared.schedule_with_run_loop(&worker.run_loop, RunLoopMode::Default)?;
    assert!(shared.set_callback(|_| {}).is_err());
    shared.unschedule_from_run_loop(&worker.run_loop, RunLoopMode::Default)?;
    shared.set_callback(|_| {})?;

    drop(local);
    drop(shared);
    worker.join();
    Ok(())
}
