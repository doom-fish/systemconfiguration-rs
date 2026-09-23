use std::ffi::c_void;

use apple_cf::cf::CFString;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RunLoopMode<'a> {
    Default,
    Common,
    Named(&'a str),
}

impl RunLoopMode<'_> {
    pub(crate) fn with_raw<R>(self, f: impl FnOnce(*mut c_void) -> R) -> R {
        match self {
            Self::Default => f(unsafe { apple_cf::raw::kCFRunLoopDefaultMode }
                .cast_mut()
                .cast()),
            Self::Common => f(unsafe { apple_cf::raw::kCFRunLoopCommonModes }
                .cast_mut()
                .cast()),
            Self::Named(name) => {
                let name = CFString::new(name);
                f(name.as_ptr())
            }
        }
    }
}
