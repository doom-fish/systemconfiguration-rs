#![allow(dead_code)]

use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

use systemconfiguration::Preferences;

pub fn fixture_dir() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data");
    fs::create_dir_all(&dir).expect("failed to create target/test-data");
    dir
}

pub fn unique_prefs_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    fixture_dir().join(format!("{name}-{nanos}.plist"))
}

pub fn temporary_preferences(name: &str) -> Preferences {
    let path = unique_prefs_path(name);
    Preferences::new("systemconfiguration-rs-tests", Some(path.to_string_lossy().as_ref()))
        .expect("failed to create temporary preferences")
}
