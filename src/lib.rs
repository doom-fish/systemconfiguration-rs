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

mod bond_interface;
mod bridge;
mod captive_network;
mod console_user;
mod dynamic_store;
mod error;
mod ffi;
mod network_configuration;
mod network_connection;
mod network_interface;
mod network_protocol;
mod network_reachability;
mod network_services;
mod network_sets;
mod preferences;
mod property_list;
mod schema;
pub mod schema_definitions;
mod system_configuration;
mod vlan_interface;

pub use bond_interface::{BondInterface, BondStatus};
pub use captive_network::CaptiveNetwork;
pub use console_user::ConsoleUser;
pub use dynamic_store::{DynamicStore, DynamicStoreRunLoopSource};
pub use error::{Result, SystemConfigurationError};
pub use network_configuration::{NetworkConfiguration, NetworkConfigurationOverview};
pub use network_connection::{
    NetworkConnection, NetworkConnectionFlags, NetworkConnectionPppStatus, NetworkConnectionStatus,
    NetworkConnectionUserPreferences,
};
pub use network_interface::{
    NetworkInterface, NetworkInterfaceMediaOptions, NetworkInterfaceMtuInfo,
};
pub use network_protocol::NetworkProtocol;
pub use network_reachability::{NetworkReachability, Reachability, ReachabilityFlags};
pub use network_services::NetworkService;
pub use network_sets::NetworkSet;
pub use preferences::{Preferences, PreferencesNotification};
pub use property_list::PropertyList;
pub use schema::{Schema, SchemaCatalog};
pub use system_configuration::{SystemConfiguration, SystemConfigurationLastError};
pub use vlan_interface::VlanInterface;

#[cfg(feature = "raw-ffi")]
pub mod raw_ffi;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;

pub mod prelude {
    pub use crate::{
        schema_definitions, BondInterface, BondStatus, CaptiveNetwork, ConsoleUser, DynamicStore,
        DynamicStoreRunLoopSource, NetworkConfiguration, NetworkConfigurationOverview,
        NetworkConnection, NetworkConnectionFlags, NetworkConnectionPppStatus,
        NetworkConnectionStatus, NetworkConnectionUserPreferences, NetworkInterface,
        NetworkInterfaceMediaOptions, NetworkInterfaceMtuInfo, NetworkProtocol,
        NetworkReachability, NetworkService, NetworkSet, Preferences, PreferencesNotification,
        PropertyList, Reachability, ReachabilityFlags, Result, Schema, SchemaCatalog,
        SystemConfiguration, SystemConfigurationError, SystemConfigurationLastError, VlanInterface,
    };
}
