use crate::{bridge, error::Result, ffi, network_services::NetworkService, PropertyList};

#[derive(Clone, Debug)]
pub struct Preferences {
    raw: bridge::OwnedHandle,
}

impl Preferences {
    pub fn new(name: &str, prefs_id: Option<&str>) -> Result<Self> {
        let name = bridge::cstring(name, "sc_preferences_create")?;
        let prefs_id = bridge::optional_cstring(prefs_id, "sc_preferences_create")?;
        let raw = unsafe {
            ffi::preferences::sc_preferences_create(
                name.as_ptr(),
                prefs_id.as_ref().map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        let raw = bridge::owned_handle_or_last("sc_preferences_create", raw)?;
        Ok(Self { raw })
    }

    pub fn lock(&self, wait: bool) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_lock(self.raw.as_ptr(), u8::from(wait)) };
        bridge::bool_result("sc_preferences_lock", ok)
    }

    pub fn commit_changes(&self) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_commit_changes(self.raw.as_ptr()) };
        bridge::bool_result("sc_preferences_commit_changes", ok)
    }

    pub fn apply_changes(&self) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_apply_changes(self.raw.as_ptr()) };
        bridge::bool_result("sc_preferences_apply_changes", ok)
    }

    pub fn unlock(&self) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_unlock(self.raw.as_ptr()) };
        bridge::bool_result("sc_preferences_unlock", ok)
    }

    pub fn synchronize(&self) {
        unsafe { ffi::preferences::sc_preferences_synchronize(self.raw.as_ptr()) };
    }

    pub fn signature(&self) -> Option<String> {
        bridge::take_optional_string(unsafe { ffi::preferences::sc_preferences_copy_signature(self.raw.as_ptr()) })
    }

    pub fn copy_key_list(&self) -> Vec<String> {
        bridge::take_string_array(unsafe { ffi::preferences::sc_preferences_copy_key_list(self.raw.as_ptr()) })
    }

    pub fn get_value(&self, key: &str) -> Result<Option<PropertyList>> {
        let key = bridge::cstring(key, "sc_preferences_get_value")?;
        let raw = unsafe { ffi::preferences::sc_preferences_get_value(self.raw.as_ptr(), key.as_ptr()) };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    pub fn add_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = bridge::cstring(key, "sc_preferences_add_value")?;
        let ok = unsafe { ffi::preferences::sc_preferences_add_value(self.raw.as_ptr(), key.as_ptr(), value.as_ptr()) };
        bridge::bool_result("sc_preferences_add_value", ok)
    }

    pub fn set_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = bridge::cstring(key, "sc_preferences_set_value")?;
        let ok = unsafe { ffi::preferences::sc_preferences_set_value(self.raw.as_ptr(), key.as_ptr(), value.as_ptr()) };
        bridge::bool_result("sc_preferences_set_value", ok)
    }

    pub fn remove_value(&self, key: &str) -> Result<()> {
        let key = bridge::cstring(key, "sc_preferences_remove_value")?;
        let ok = unsafe { ffi::preferences::sc_preferences_remove_value(self.raw.as_ptr(), key.as_ptr()) };
        bridge::bool_result("sc_preferences_remove_value", ok)
    }

    pub fn path_create_unique_child(&self, prefix: &str) -> Result<Option<String>> {
        let prefix = bridge::cstring(prefix, "sc_preferences_path_create_unique_child")?;
        Ok(bridge::take_optional_string(unsafe {
            ffi::preferences::sc_preferences_path_create_unique_child(self.raw.as_ptr(), prefix.as_ptr())
        }))
    }

    pub fn path_get_value(&self, path: &str) -> Result<Option<PropertyList>> {
        let path = bridge::cstring(path, "sc_preferences_path_get_value")?;
        let raw = unsafe { ffi::preferences::sc_preferences_path_get_value(self.raw.as_ptr(), path.as_ptr()) };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    pub fn path_get_link(&self, path: &str) -> Result<Option<String>> {
        let path = bridge::cstring(path, "sc_preferences_path_get_link")?;
        Ok(bridge::take_optional_string(unsafe {
            ffi::preferences::sc_preferences_path_get_link(self.raw.as_ptr(), path.as_ptr())
        }))
    }

    pub fn path_set_value(&self, path: &str, value: &PropertyList) -> Result<()> {
        let path = bridge::cstring(path, "sc_preferences_path_set_value")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_path_set_value(self.raw.as_ptr(), path.as_ptr(), value.as_ptr())
        };
        bridge::bool_result("sc_preferences_path_set_value", ok)
    }

    pub fn path_set_link(&self, path: &str, link: &str) -> Result<()> {
        let path = bridge::cstring(path, "sc_preferences_path_set_link")?;
        let link = bridge::cstring(link, "sc_preferences_path_set_link")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_path_set_link(self.raw.as_ptr(), path.as_ptr(), link.as_ptr())
        };
        bridge::bool_result("sc_preferences_path_set_link", ok)
    }

    pub fn path_remove_value(&self, path: &str) -> Result<()> {
        let path = bridge::cstring(path, "sc_preferences_path_remove_value")?;
        let ok = unsafe { ffi::preferences::sc_preferences_path_remove_value(self.raw.as_ptr(), path.as_ptr()) };
        bridge::bool_result("sc_preferences_path_remove_value", ok)
    }

    pub fn set_computer_name(&self, name: Option<&str>) -> Result<()> {
        let name = bridge::optional_cstring(name, "sc_preferences_set_computer_name")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_computer_name(
                self.raw.as_ptr(),
                name.as_ref().map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        bridge::bool_result("sc_preferences_set_computer_name", ok)
    }

    pub fn set_local_host_name(&self, name: Option<&str>) -> Result<()> {
        let name = bridge::optional_cstring(name, "sc_preferences_set_local_host_name")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_local_host_name(
                self.raw.as_ptr(),
                name.as_ref().map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        bridge::bool_result("sc_preferences_set_local_host_name", ok)
    }

    pub fn network_services(&self) -> Vec<NetworkService> {
        NetworkService::copy_all(self)
    }

    pub(crate) fn as_ptr(&self) -> bridge::RawHandle {
        self.raw.as_ptr()
    }
}
