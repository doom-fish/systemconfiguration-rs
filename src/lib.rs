#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::borrow_as_ptr,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate,
    clippy::similar_names,
    clippy::use_self
)]
#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "macos"))]
compile_error!("systemconfiguration only supports macOS");

mod cf;
mod dynamic_store;
mod error;
mod ffi;
mod network_interface;
mod preferences;
mod reachability;

pub use cf::PropertyList;
pub use dynamic_store::DynamicStore;
pub use error::{Result, SystemConfigurationError};
pub use network_interface::NetworkInterface;
pub use preferences::{NetworkService, Preferences};
pub use reachability::{Reachability, ReachabilityFlags};

pub mod prelude {
    pub use crate::{
        DynamicStore, NetworkInterface, NetworkService, Preferences, PropertyList, Reachability,
        ReachabilityFlags, Result, SystemConfigurationError,
    };
}
