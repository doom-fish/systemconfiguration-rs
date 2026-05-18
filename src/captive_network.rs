use crate::{bridge, error::Result, ffi};

#[derive(Clone, Copy, Debug, Default)]
/// Wraps the CaptiveNetwork support APIs in SystemConfiguration.
pub struct CaptiveNetwork;

impl CaptiveNetwork {
    /// Wraps `SCCaptiveNetworkCopySupportedInterfaces`.
    pub fn supported_interfaces() -> Vec<String> {
        bridge::take_string_array(unsafe {
            ffi::captive_network::sc_captive_network_copy_supported_interfaces()
        })
    }

    /// Wraps `SCCaptiveNetworkSetSupportedSSIDs`.
    pub fn set_supported_ssids<S: AsRef<str>>(values: &[S]) -> Result<()> {
        let values = bridge::CStringArray::new(values, "sc_captive_network_set_supported_ssids")?;
        let ok = unsafe {
            ffi::captive_network::sc_captive_network_set_supported_ssids(
                values.as_ptr(),
                values.count(),
            )
        };
        bridge::bool_result("sc_captive_network_set_supported_ssids", ok)
    }

    /// Wraps `SCCaptiveNetworkMarkPortalOnline`.
    pub fn mark_portal_online(interface_name: &str) -> Result<()> {
        let interface_name =
            bridge::cstring(interface_name, "sc_captive_network_mark_portal_online")?;
        let ok = unsafe {
            ffi::captive_network::sc_captive_network_mark_portal_online(interface_name.as_ptr())
        };
        bridge::bool_result("sc_captive_network_mark_portal_online", ok)
    }

    /// Wraps `SCCaptiveNetworkMarkPortalOffline`.
    pub fn mark_portal_offline(interface_name: &str) -> Result<()> {
        let interface_name =
            bridge::cstring(interface_name, "sc_captive_network_mark_portal_offline")?;
        let ok = unsafe {
            ffi::captive_network::sc_captive_network_mark_portal_offline(interface_name.as_ptr())
        };
        bridge::bool_result("sc_captive_network_mark_portal_offline", ok)
    }
}
