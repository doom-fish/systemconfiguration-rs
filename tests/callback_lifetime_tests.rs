mod common;

use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

use systemconfiguration::{
    CFRunLoop, DispatchQoS, DispatchQueue, DynamicStore, NetworkConnection, Preferences,
    Reachability, RunLoopMode,
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
                .send(CFRunLoop::current())
                .expect("send run loop");
            while stop_rx.try_recv().is_err() {
                let _ = CFRunLoop::run_in_default_mode(Duration::from_millis(20), false);
            }
        });
        let run_loop = run_loop_rx.recv().expect("receive run loop");
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
    source.schedule(&CFRunLoop::current(), RunLoopMode::Default)?;

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
    prefs.set_dispatch_queue(&DispatchQueue::new(
        "systemconfiguration-rs.test-prefs-queue",
        DispatchQoS::Utility,
    ))?;
    drop(prefs);
    assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    Ok(())
}

#[test]
fn preferences_lock_returns_a_guard_or_an_access_error() -> Result<(), Box<dyn std::error::Error>> {
    let prefs = common::temporary_preferences("lock-guard");
    if unsafe { libc::geteuid() } != 0 {
        let error = prefs
            .lock(false)
            .expect_err("locking needs root or an authorization");
        assert_eq!(error.code, 1003);
        return Ok(());
    }

    let path = common::unique_prefs_path("lock-guard-root");
    let first = Preferences::new(
        "systemconfiguration-rs.lock-first",
        Some(path.to_string_lossy().as_ref()),
    )?;
    let second = Preferences::new(
        "systemconfiguration-rs.lock-second",
        Some(path.to_string_lossy().as_ref()),
    )?;
    let guard = first.lock(false)?;
    assert!(second.lock(false).is_err());
    drop(guard);
    second.lock(false)?.unlock()?;
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
    reachability.set_callback(move |_| {
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
        reachability.set_callback(move |_| {
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
    reachability.set_callback(move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
    })?;
    reachability.set_dispatch_queue(&queue)?;
    assert!(wait_for(|| hits.load(Ordering::SeqCst) > before));
    drop(reachability);
    assert!(wait_for(|| Arc::strong_count(&hits) == 1));
    Ok(())
}

#[test]
fn reachability_callbacks_can_be_replaced_while_scheduled_on_another_thread_or_a_queue(
) -> Result<(), Box<dyn std::error::Error>> {
    let worker = RunLoopThread::spawn();
    let (first, first_captured) = witness();
    let mut on_worker = Reachability::with_name("localhost")?;
    on_worker.set_callback(move |_| {
        let _ = &first_captured;
    })?;
    on_worker.schedule_with_run_loop(&worker.run_loop, RunLoopMode::Default)?;
    on_worker.set_callback(|_| {})?;
    assert!(wait_for(|| Arc::strong_count(&first) == 1));

    let (second, second_captured) = witness();
    let mut on_queue = Reachability::with_name("localhost")?;
    on_queue.set_dispatch_queue(&DispatchQueue::new(
        "systemconfiguration-rs.test-replace",
        DispatchQoS::Utility,
    ))?;
    on_queue.set_callback(move |_| {
        let _ = &second_captured;
    })?;
    on_queue.set_callback(|_| {})?;
    assert!(wait_for(|| Arc::strong_count(&second) == 1));

    drop(on_worker);
    drop(on_queue);
    worker.join();
    Ok(())
}

struct DropFlag(Arc<AtomicBool>);

impl Drop for DropFlag {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

thread_local! {
    static OWN_REGISTRATION: RefCell<Option<Reachability>> = const { RefCell::new(None) };
}

#[test]
fn a_reachability_callback_can_drop_its_own_registration() -> Result<(), Box<dyn std::error::Error>>
{
    let dropped = Arc::new(AtomicBool::new(false));
    let dropped_while_running = Arc::new(AtomicBool::new(false));
    let calls = Arc::new(AtomicUsize::new(0));
    let flag = DropFlag(Arc::clone(&dropped));
    let observed = Arc::clone(&dropped);
    let while_running = Arc::clone(&dropped_while_running);
    let counter = Arc::clone(&calls);
    let mut reachability = Reachability::with_name("localhost")?;
    reachability.set_callback(move |_| {
        let _ = &flag;
        counter.fetch_add(1, Ordering::SeqCst);
        let own = OWN_REGISTRATION.with(|slot| slot.borrow_mut().take());
        drop(own);
        while_running.fetch_or(observed.load(Ordering::SeqCst), Ordering::SeqCst);
    })?;
    reachability.schedule_with_run_loop(&CFRunLoop::current(), RunLoopMode::Default)?;
    OWN_REGISTRATION.with(|slot| *slot.borrow_mut() = Some(reachability));

    assert!(wait_for(|| dropped.load(Ordering::SeqCst)));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(!dropped_while_running.load(Ordering::SeqCst));
    assert!(OWN_REGISTRATION.with(|slot| slot.borrow().is_none()));
    Ok(())
}

#[test]
fn a_reachability_callback_can_replace_its_own_registration(
) -> Result<(), Box<dyn std::error::Error>> {
    let dropped = Arc::new(AtomicBool::new(false));
    let dropped_while_running = Arc::new(AtomicBool::new(false));
    let replaced = Arc::new(AtomicBool::new(false));
    let flag = DropFlag(Arc::clone(&dropped));
    let observed = Arc::clone(&dropped);
    let while_running = Arc::clone(&dropped_while_running);
    let replaced_flag = Arc::clone(&replaced);
    let mut reachability = Reachability::with_name("localhost")?;
    reachability.set_callback(move |_| {
        let _ = &flag;
        let replacement = OWN_REGISTRATION.with(|slot| {
            slot.borrow_mut()
                .as_mut()
                .map(|own| own.set_callback(|_| {}))
        });
        replaced_flag.fetch_or(matches!(replacement, Some(Ok(()))), Ordering::SeqCst);
        while_running.fetch_or(observed.load(Ordering::SeqCst), Ordering::SeqCst);
    })?;
    reachability.schedule_with_run_loop(&CFRunLoop::current(), RunLoopMode::Default)?;
    OWN_REGISTRATION.with(|slot| *slot.borrow_mut() = Some(reachability));

    assert!(wait_for(|| dropped.load(Ordering::SeqCst)));
    assert!(replaced.load(Ordering::SeqCst));
    assert!(!dropped_while_running.load(Ordering::SeqCst));
    let own = OWN_REGISTRATION.with(|slot| slot.borrow_mut().take());
    assert!(own.is_some());
    drop(own);
    Ok(())
}
