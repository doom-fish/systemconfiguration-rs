use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{bridge, error::Result, ffi};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
/// Wraps a summary built from SystemConfiguration network-configuration APIs.
pub struct NetworkConfigurationOverview {
    /// Wraps the interface count derived from `SCNetworkInterfaceCopyAll`.
    pub interface_count: usize,
    /// Wraps the service count derived from `SCNetworkServiceCopyAll`.
    pub service_count: usize,
    /// Wraps the set count derived from `SCNetworkSetCopyAll`.
    pub set_count: usize,
    /// Wraps the current set name derived from `SCNetworkSetCopyCurrent`.
    pub current_set_name: Option<String>,
    /// Wraps the current set identifier derived from `SCNetworkSetCopyCurrent`.
    pub current_set_id: Option<String>,
    /// Wraps interface-type symbols from `SCNetworkInterfaceCopyAll`.
    pub interface_types: BTreeMap<String, String>,
    /// Wraps protocol-type symbols from `SCNetworkServiceCopyProtocols`.
    pub protocol_types: BTreeMap<String, String>,
}

#[derive(Clone, Copy, Debug, Default)]
/// Provides overview helpers for SystemConfiguration network-configuration APIs.
pub struct NetworkConfiguration;

impl NetworkConfiguration {
    /// Wraps the bridge overview assembled from `SCNetworkInterfaceCopyAll`, `SCNetworkServiceCopyAll`, and `SCNetworkSetCopyAll`.
    pub fn overview() -> Result<NetworkConfigurationOverview> {
        bridge::parse_json("sc_network_configuration_copy_overview", unsafe {
            ffi::network_configuration::sc_network_configuration_copy_overview()
        })
    }
}
