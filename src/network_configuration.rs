use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{bridge, error::Result, ffi};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct NetworkConfigurationOverview {
    pub interface_count: usize,
    pub service_count: usize,
    pub set_count: usize,
    pub current_set_name: Option<String>,
    pub current_set_id: Option<String>,
    pub interface_types: BTreeMap<String, String>,
    pub protocol_types: BTreeMap<String, String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NetworkConfiguration;

impl NetworkConfiguration {
    pub fn overview() -> Result<NetworkConfigurationOverview> {
        bridge::parse_json(
            "sc_network_configuration_copy_overview",
            unsafe { ffi::network_configuration::sc_network_configuration_copy_overview() },
        )
    }
}
