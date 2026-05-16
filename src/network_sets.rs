use crate::{
    bridge,
    error::Result,
    ffi,
    network_interface::NetworkInterface,
    network_services::NetworkService,
    preferences::Preferences,
};

#[derive(Clone, Debug)]
pub struct NetworkSet {
    raw: bridge::OwnedHandle,
}

impl NetworkSet {
    pub fn type_id() -> u64 {
        unsafe { ffi::network_sets::sc_network_set_get_type_id() }
    }

    pub fn copy_all(preferences: &Preferences) -> Vec<Self> {
        let raw = unsafe { ffi::network_sets::sc_network_set_copy_all(preferences.as_ptr()) };
        bridge::take_handle_array(raw, Self::from_owned_handle)
    }

    pub fn create(preferences: &Preferences) -> Result<Self> {
        let raw = unsafe { ffi::network_sets::sc_network_set_create(preferences.as_ptr()) };
        let raw = bridge::owned_handle_or_last("sc_network_set_create", raw)?;
        Ok(Self { raw })
    }

    pub fn copy(preferences: &Preferences, set_id: &str) -> Result<Option<Self>> {
        let set_id = bridge::cstring(set_id, "sc_network_set_copy")?;
        let raw = unsafe { ffi::network_sets::sc_network_set_copy(preferences.as_ptr(), set_id.as_ptr()) };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(Self::from_owned_handle))
    }

    pub fn copy_current(preferences: &Preferences) -> Option<Self> {
        unsafe { bridge::OwnedHandle::from_raw(ffi::network_sets::sc_network_set_copy_current(preferences.as_ptr())) }
            .map(Self::from_owned_handle)
    }

    pub fn copy_services(&self) -> Vec<NetworkService> {
        let raw = unsafe { ffi::network_sets::sc_network_set_copy_services(self.raw.as_ptr()) };
        bridge::take_handle_array(raw, NetworkService::from_owned_handle)
    }

    pub fn name(&self) -> Option<String> {
        bridge::take_optional_string(unsafe { ffi::network_sets::sc_network_set_copy_name(self.raw.as_ptr()) })
    }

    pub fn set_id(&self) -> Option<String> {
        bridge::take_optional_string(unsafe { ffi::network_sets::sc_network_set_copy_set_id(self.raw.as_ptr()) })
    }

    pub fn service_order(&self) -> Vec<String> {
        bridge::take_string_array(unsafe {
            ffi::network_sets::sc_network_set_copy_service_order(self.raw.as_ptr())
        })
    }

    pub fn contains_interface(&self, interface: &NetworkInterface) -> bool {
        unsafe {
            ffi::network_sets::sc_network_set_contains_interface(self.raw.as_ptr(), interface.as_ptr()) != 0
        }
    }

    pub fn add_service(&self, service: &NetworkService) -> Result<()> {
        let ok = unsafe { ffi::network_sets::sc_network_set_add_service(self.raw.as_ptr(), service.as_ptr()) };
        bridge::bool_result("sc_network_set_add_service", ok)
    }

    pub fn remove(&self) -> Result<()> {
        let ok = unsafe { ffi::network_sets::sc_network_set_remove(self.raw.as_ptr()) };
        bridge::bool_result("sc_network_set_remove", ok)
    }

    pub fn remove_service(&self, service: &NetworkService) -> Result<()> {
        let ok = unsafe {
            ffi::network_sets::sc_network_set_remove_service(self.raw.as_ptr(), service.as_ptr())
        };
        bridge::bool_result("sc_network_set_remove_service", ok)
    }

    pub fn set_current(&self) -> Result<()> {
        let ok = unsafe { ffi::network_sets::sc_network_set_set_current(self.raw.as_ptr()) };
        bridge::bool_result("sc_network_set_set_current", ok)
    }

    pub fn set_name(&self, name: Option<&str>) -> Result<()> {
        let name = bridge::optional_cstring(name, "sc_network_set_set_name")?;
        let ok = unsafe {
            ffi::network_sets::sc_network_set_set_name(
                self.raw.as_ptr(),
                name.as_ref().map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        bridge::bool_result("sc_network_set_set_name", ok)
    }

    pub fn set_service_order<S: AsRef<str>>(&self, values: &[S]) -> Result<()> {
        let values = bridge::CStringArray::new(values, "sc_network_set_set_service_order")?;
        let ok = unsafe {
            ffi::network_sets::sc_network_set_set_service_order(
                self.raw.as_ptr(),
                values.as_ptr(),
                values.count(),
            )
        };
        bridge::bool_result("sc_network_set_set_service_order", ok)
    }

    pub(crate) fn from_owned_handle(raw: bridge::OwnedHandle) -> Self {
        Self { raw }
    }
}
