use libzfs_sys::raw::{
    libzfs_error_description, libzfs_fini, libzfs_handle_t, libzfs_init, nvlist_t, nvpair_t,
    zfs_create, zfs_destroy, zfs_open, zfs_type_t_ZFS_TYPE_FILESYSTEM,
};

use crate::nvlist::Nvlist;

pub struct LibZfs {
    inner: *mut libzfs_handle_t,
}

impl Drop for LibZfs {
    fn drop(&mut self) {
        unsafe { libzfs_fini(self.inner) }
    }
}

impl LibZfs {
    pub fn new() -> Option<Self> {
        let inner = unsafe { libzfs_init() };
        if inner.is_null() {
            None
        } else {
            Some(Self { inner })
        }
    }

    pub fn create_filesystem(&self, name: &str) -> Result<String, String> {
        let cname = std::ffi::CString::new(name).expect("Failed to allocate memory");
        let ret = unsafe {
            zfs_create(
                self.inner,
                cname.as_ptr(),
                zfs_type_t_ZFS_TYPE_FILESYSTEM,
                std::ptr::null_mut(),
            )
        };

        if ret != 0 {
            unsafe {
                Err(
                    std::ffi::CStr::from_ptr(libzfs_error_description(self.inner))
                        .to_str()
                        .unwrap()
                        .to_string(),
                )
            }
        } else {
            Ok(name.to_string())
        }
    }

    pub fn destroy_filesystem(&self, name: &str) -> Result<(), String> {
        let cname = std::ffi::CString::new(name).unwrap();

        let zhp = unsafe {
            zfs_open(
                self.inner,
                cname.as_ptr(),
                zfs_type_t_ZFS_TYPE_FILESYSTEM as i32,
            )
        };

        if zhp.is_null() {
            unsafe {
                return Err(
                    std::ffi::CStr::from_ptr(libzfs_error_description(self.inner))
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
            }
        }

        let ret = unsafe { zfs_destroy(zhp, true as u32) };

        if ret != 0 {
            unsafe {
                Err(
                    std::ffi::CStr::from_ptr(libzfs_error_description(self.inner))
                        .to_str()
                        .unwrap()
                        .to_string(),
                )
            }
        } else {
            Ok(())
        }
    }
}

#[test]
fn zfs_create_destroy() {
    // a pool "test" is assumed to be existing
    let hdl = LibZfs::new().unwrap();
    let result = hdl.create_filesystem("test/delme").unwrap();
    let result = hdl.destroy_filesystem("test/delme").unwrap();
}
