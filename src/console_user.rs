use crate::{bridge, ffi};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsoleUser {
    name: String,
    uid: u32,
    gid: u32,
}

impl ConsoleUser {
    pub fn current() -> Option<Self> {
        let raw = unsafe { ffi::console_user::sc_console_user_copy_current() };
        let raw = unsafe { bridge::OwnedHandle::from_raw(raw) }?;
        let name = bridge::take_optional_string(unsafe {
            ffi::console_user::sc_console_user_copy_name(raw.as_ptr())
        })?;
        let uid = unsafe { ffi::console_user::sc_console_user_get_uid(raw.as_ptr()) };
        let gid = unsafe { ffi::console_user::sc_console_user_get_gid(raw.as_ptr()) };
        Some(Self { name, uid, gid })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> u32 {
        self.gid
    }
}
