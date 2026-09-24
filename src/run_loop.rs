use std::ffi::c_void;

use apple_cf::cf::{CFRunLoop, CFString};

use crate::{
    error::Result, schema_definitions::SC_STATUS_INVALID_ARGUMENT, SystemConfigurationError,
};

pub(crate) fn require_this_thread_or_main(
    run_loop: &CFRunLoop,
    function: &'static str,
) -> Result<()> {
    if *run_loop == CFRunLoop::current() || *run_loop == CFRunLoop::main() {
        return Ok(());
    }
    Err(SystemConfigurationError::new(
        function,
        SC_STATUS_INVALID_ARGUMENT,
        "SystemConfiguration runs this run-loop source's schedule and cancel callouts without locking, and Core Foundation cancels a source on the thread that exits or frees its run loop; schedule it on this thread's run loop or the main run loop, or use set_dispatch_queue",
    ))
}

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
