//use crate::helpers::{Display, };
use crate::wl::core;

/*struct Interface {
    ptr: *mut std::ffi::c_void,
    name: &'static str,
}*/

pub trait FromWLInterface where Self: std::marker::Sized {
    //type Err;
    //fn get_type_str() -> &'static str;
    fn get_interface() -> *const core::wl_interface;
    unsafe fn from_void_handle(ptr: *mut std::ffi::c_void) -> Result<Self, &'static str>;
}

pub struct Compositor {
    pub(crate) ptr: *mut core::wl_compositor,
}

impl FromWLInterface for Compositor {
    fn get_interface() -> *const core::wl_interface {
        unsafe {
            &core::wl_compositor_interface as *const core::wl_interface
        }
    }

    unsafe fn from_void_handle(ptr: *mut std::ffi::c_void) -> Result<Self, &'static str> {
        //Ok(Compositor{})
        match ptr.is_null() {
            true => Err("Wayland compositor returned null handle on compositor create request"),
            false => Ok(Compositor { ptr: ptr as *mut core::wl_compositor })
        }
    }
}

/*unsafe impl Sync for Compositor {
    //
}*/

pub struct Surface {
    pub(crate) ptr: *mut core::wl_surface,
}

#[allow(non_camel_case_types)]
//#[repr(C)] // 
pub struct Registry {
    pub(crate) ptr: *mut core::wl_registry,
}

#[allow(non_camel_case_types)]
//#[repr(C)]
pub struct Display {
    pub(crate) ptr: *mut core::wl_display,
}

pub struct Shm {
    pub(crate) ptr: *mut core::wl_shm,
}

impl FromWLInterface for Shm {
    fn get_interface() -> *const core::wl_interface {
        unsafe {
            &core::wl_shm_interface as *const core::wl_interface
        }
    }

    unsafe fn from_void_handle(ptr: *mut std::ffi::c_void) -> Result<Self, &'static str> {
        match ptr.is_null() {
            true => Err("Wayland compositor returned null handle on compositor create request"),
            false => Ok(Shm { ptr: ptr as *mut core::wl_shm })
        }
    }
}

/*use std::rc::Arc;

/// Used as a convenience object to hold references to wl
/// globals. Follows interior mutability pattern
pub struct Globals {
    //
}

struct GuardedGlobals {

}

/*trait Global {
    pub fn 
}*/

impl Globals {
    pub fn new() -> Globals {
        Globals {}
    }

    /*pub fn acquire<T>() -> Result<&mut T, &str> where T: Global {
        //
    }*/

    /*pub fn release<T>(global: T) where T: Global {
        //
    }*/

    pub fn display() -> &mut Display {
        self.
    }
}*/
