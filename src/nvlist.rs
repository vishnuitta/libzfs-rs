use libzfs_sys::raw::{
    fnvlist_lookup_boolean_value, fnvlist_lookup_int32, nvlist_alloc, nvlist_t, NV_UNIQUE_NAME,
};
use std::any::{Any, TypeId};

pub struct Nvlist {
    inner: *mut nvlist_t,
}

impl Nvlist {
    pub fn new() -> Option<Self> {
        let mut nvlist_ptr: *mut nvlist_t = std::ptr::null_mut();
        let rc = unsafe { nvlist_alloc(&mut nvlist_ptr, NV_UNIQUE_NAME, 0) };

        if rc != 0 {
            None
        } else {
            Some(Self { inner: nvlist_ptr })
        }
    }

    pub fn as_ptr(&mut self) -> *mut nvlist_t {
        self.inner
    }

}
