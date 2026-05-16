use std::{
    ffi::{c_char, c_void, CString},
    ptr::NonNull,
};

use serde::de::DeserializeOwned;

use crate::{error::Result, ffi, SystemConfigurationError};

pub(crate) type RawHandle = *mut c_void;

#[derive(Debug)]
pub(crate) struct OwnedHandle(NonNull<c_void>);

impl OwnedHandle {
    pub(crate) unsafe fn from_raw(raw: RawHandle) -> Option<Self> {
        NonNull::new(raw).map(Self)
    }

    pub(crate) fn as_ptr(&self) -> RawHandle {
        self.0.as_ptr()
    }
}

impl Clone for OwnedHandle {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::core::sc_handle_retain(self.as_ptr()) };
        unsafe { Self::from_raw(raw) }.expect("sc_handle_retain returned null")
    }
}

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        unsafe { ffi::core::sc_handle_release(self.as_ptr()) };
    }
}

pub(crate) fn owned_handle_or_last(function: &'static str, raw: RawHandle) -> Result<OwnedHandle> {
    unsafe { OwnedHandle::from_raw(raw) }.ok_or_else(|| SystemConfigurationError::last(function))
}

pub(crate) fn bool_result(function: &'static str, value: u8) -> Result<()> {
    if value == 0 {
        Err(SystemConfigurationError::last(function))
    } else {
        Ok(())
    }
}

pub(crate) fn cstring(value: &str, function: &'static str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        SystemConfigurationError::null(function, "strings passed over FFI cannot contain interior NUL bytes")
    })
}

pub(crate) fn optional_cstring(
    value: Option<&str>,
    function: &'static str,
) -> Result<Option<CString>> {
    value.map(|value| cstring(value, function)).transpose()
}

pub(crate) struct CStringArray {
    strings: Vec<CString>,
    pointers: Vec<*const c_char>,
}

impl CStringArray {
    pub(crate) fn new<S: AsRef<str>>(values: &[S], function: &'static str) -> Result<Self> {
        let strings = values
            .iter()
            .map(|value| cstring(value.as_ref(), function))
            .collect::<Result<Vec<_>>>()?;
        let pointers = strings.iter().map(|value| value.as_ptr()).collect();
        Ok(Self { strings, pointers })
    }

    pub(crate) fn as_ptr(&self) -> *const *const c_char {
        if self.pointers.is_empty() {
            std::ptr::null()
        } else {
            self.pointers.as_ptr()
        }
    }

    pub(crate) fn count(&self) -> isize {
        isize::try_from(self.strings.len()).expect("cstring array length exceeded isize")
    }
}

pub(crate) struct HandleArray {
    handles: Vec<RawHandle>,
}

impl HandleArray {
    pub(crate) fn new<T, F>(values: &[T], mut map: F) -> Self
    where
        F: FnMut(&T) -> RawHandle,
    {
        let handles = values.iter().map(&mut map).collect();
        Self { handles }
    }

    pub(crate) fn as_ptr(&self) -> *const RawHandle {
        if self.handles.is_empty() {
            std::ptr::null()
        } else {
            self.handles.as_ptr()
        }
    }

    pub(crate) fn count(&self) -> isize {
        isize::try_from(self.handles.len()).expect("handle array length exceeded isize")
    }
}

fn read_string(raw: RawHandle) -> String {
    let len = unsafe { ffi::core::sc_string_len(raw) };
    let len = usize::try_from(len).expect("negative Swift string length");
    let mut buffer = vec![0_u8; len.max(1)];
    let written = unsafe {
        ffi::core::sc_string_copy_utf8(
            raw,
            buffer.as_mut_ptr(),
            isize::try_from(buffer.len()).expect("buffer length exceeded isize"),
        )
    };
    let written = usize::try_from(written).expect("negative Swift string write length");
    String::from_utf8_lossy(&buffer[..written]).into_owned()
}

pub(crate) fn take_optional_string(raw: RawHandle) -> Option<String> {
    if raw.is_null() {
        return None;
    }

    let string = read_string(raw);
    unsafe { ffi::core::sc_handle_release(raw) };
    Some(string)
}

pub(crate) fn take_string_array(raw: RawHandle) -> Vec<String> {
    if raw.is_null() {
        return Vec::new();
    }

    let count = unsafe { ffi::core::sc_array_count(raw) };
    let count = usize::try_from(count).expect("negative Swift array length");
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        let item = unsafe {
            ffi::core::sc_array_get(
                raw,
                isize::try_from(index).expect("array index exceeded isize"),
            )
        };
        if let Some(value) = take_optional_string(item) {
            values.push(value);
        }
    }
    unsafe { ffi::core::sc_handle_release(raw) };
    values
}

pub(crate) fn take_handle_array<T, F>(raw: RawHandle, mut map: F) -> Vec<T>
where
    F: FnMut(OwnedHandle) -> T,
{
    if raw.is_null() {
        return Vec::new();
    }

    let count = unsafe { ffi::core::sc_array_count(raw) };
    let count = usize::try_from(count).expect("negative Swift array length");
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        let item = unsafe {
            ffi::core::sc_array_get(
                raw,
                isize::try_from(index).expect("array index exceeded isize"),
            )
        };
        if let Some(handle) = unsafe { OwnedHandle::from_raw(item) } {
            values.push(map(handle));
        }
    }
    unsafe { ffi::core::sc_handle_release(raw) };
    values
}

pub(crate) fn parse_json<T>(function: &'static str, raw: RawHandle) -> Result<T>
where
    T: DeserializeOwned,
{
    let json = take_optional_string(raw)
        .ok_or_else(|| SystemConfigurationError::null(function, "bridge returned null JSON payload"))?;
    serde_json::from_str(&json)
        .map_err(|error| SystemConfigurationError::new(function, 0, format!("failed to parse bridge JSON: {error}")))
}
