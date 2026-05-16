use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr::NonNull,
};

use crate::{error::Result, ffi, SystemConfigurationError};

const CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;

#[derive(Debug)]
pub(crate) struct OwnedCFType(NonNull<c_void>);

impl OwnedCFType {
    pub(crate) fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr().cast_const()
    }

    pub(crate) unsafe fn from_create_rule(raw: *const c_void) -> Option<Self> {
        NonNull::new(raw.cast_mut()).map(Self)
    }

    pub(crate) unsafe fn from_get_rule(raw: *const c_void) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            let retained = ffi::CFRetain(raw);
            Self::from_create_rule(retained)
        }
    }
}

impl Clone for OwnedCFType {
    fn clone(&self) -> Self {
        let retained = unsafe { ffi::CFRetain(self.as_ptr()) };
        unsafe { Self::from_create_rule(retained) }.expect("CFRetain returned null")
    }
}

impl Drop for OwnedCFType {
    fn drop(&mut self) {
        unsafe { ffi::CFRelease(self.as_ptr()) };
    }
}

#[derive(Clone, Debug)]
pub struct PropertyList(OwnedCFType);

impl PropertyList {
    pub fn from_string(value: &str) -> Result<Self> {
        Ok(Self(cfstring_from_str(value)?))
    }

    pub fn from_bool(value: bool) -> Self {
        let raw = unsafe {
            if value {
                ffi::kCFBooleanTrue
            } else {
                ffi::kCFBooleanFalse
            }
        };
        let retained = unsafe { ffi::CFRetain(raw) };
        let owned = unsafe { OwnedCFType::from_create_rule(retained) }
            .expect("CFRetain(kCFBoolean*) returned null");
        Self(owned)
    }

    pub fn description(&self) -> Result<String> {
        copy_description(self.as_ptr())
    }

    pub(crate) fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }

    pub(crate) unsafe fn from_create_rule(raw: *const c_void) -> Option<Self> {
        OwnedCFType::from_create_rule(raw).map(Self)
    }
}

pub(crate) fn cfstring_from_str(value: &str) -> Result<OwnedCFType> {
    let c_string = CString::new(value).map_err(|_| {
        SystemConfigurationError::null(
            "CFStringCreateWithCString",
            "strings passed to CoreFoundation cannot contain interior NUL bytes",
        )
    })?;
    let cf_string = unsafe {
        ffi::CFStringCreateWithCString(std::ptr::null(), c_string.as_ptr(), CF_STRING_ENCODING_UTF8)
    };
    unsafe { OwnedCFType::from_create_rule(cf_string) }.ok_or_else(|| {
        SystemConfigurationError::null(
            "CFStringCreateWithCString",
            format!("could not create CFString for {value:?}"),
        )
    })
}

pub(crate) fn cfstring_to_string(cf_string: *const c_void) -> Result<String> {
    if cf_string.is_null() {
        return Err(SystemConfigurationError::null(
            "CFStringGetCString",
            "received null CFStringRef",
        ));
    }

    let length = unsafe { ffi::CFStringGetLength(cf_string) };
    let capacity =
        unsafe { ffi::CFStringGetMaximumSizeForEncoding(length, CF_STRING_ENCODING_UTF8) } + 1;
    let mut buffer = vec![0_u8; usize::try_from(capacity).expect("negative CFString capacity")];

    let ok = unsafe {
        ffi::CFStringGetCString(
            cf_string,
            buffer.as_mut_ptr().cast::<c_char>(),
            capacity,
            CF_STRING_ENCODING_UTF8,
        )
    };
    if ok == 0 {
        return Err(SystemConfigurationError::null(
            "CFStringGetCString",
            "CoreFoundation rejected UTF-8 conversion",
        ));
    }

    let c_string = CStr::from_bytes_until_nul(&buffer).map_err(|_| {
        SystemConfigurationError::null(
            "CFStringGetCString",
            "CoreFoundation returned a non-NUL-terminated string",
        )
    })?;
    Ok(c_string.to_string_lossy().into_owned())
}

pub(crate) fn optional_cfstring_to_string(cf_string: *const c_void) -> Result<Option<String>> {
    if cf_string.is_null() {
        Ok(None)
    } else {
        cfstring_to_string(cf_string).map(Some)
    }
}

pub(crate) fn copy_description(value: *const c_void) -> Result<String> {
    let description = unsafe { ffi::CFCopyDescription(value) };
    let description = unsafe { OwnedCFType::from_create_rule(description) }.ok_or_else(|| {
        SystemConfigurationError::null("CFCopyDescription", "received null description")
    })?;
    cfstring_to_string(description.as_ptr())
}

pub(crate) fn cfarray_to_owned(array: *const c_void) -> Vec<OwnedCFType> {
    if array.is_null() {
        return Vec::new();
    }

    let count = unsafe { ffi::CFArrayGetCount(array) };
    (0..count)
        .filter_map(|index| {
            let raw = unsafe { ffi::CFArrayGetValueAtIndex(array, index) };
            unsafe { OwnedCFType::from_get_rule(raw) }
        })
        .collect()
}

pub(crate) fn cfarray_to_strings(array: *const c_void) -> Result<Vec<String>> {
    cfarray_to_owned(array)
        .into_iter()
        .map(|value| cfstring_to_string(value.as_ptr()))
        .collect()
}

pub(crate) fn cfarray_from_strings<S: AsRef<str>>(values: &[S]) -> Result<Option<OwnedCFType>> {
    if values.is_empty() {
        return Ok(None);
    }

    let strings = values
        .iter()
        .map(|value| cfstring_from_str(value.as_ref()))
        .collect::<Result<Vec<_>>>()?;
    let raw_values = strings.iter().map(OwnedCFType::as_ptr).collect::<Vec<_>>();
    let num_values = isize::try_from(raw_values.len()).map_err(|_| {
        SystemConfigurationError::null("CFArrayCreate", "too many strings for CFArray")
    })?;

    let array = unsafe {
        ffi::CFArrayCreate(
            std::ptr::null(),
            raw_values.as_ptr(),
            num_values,
            std::ptr::null(),
        )
    };
    Ok(unsafe { OwnedCFType::from_create_rule(array) })
}

pub(crate) fn current_run_loop() -> *const c_void {
    unsafe { ffi::CFRunLoopGetCurrent() }
}

pub(crate) fn default_run_loop_mode() -> *const c_void {
    unsafe { ffi::kCFRunLoopDefaultMode }
}
