use crate::wl::core;
use crate::wl::constants;
use crate::wl::types::*;

use std::sync::Arc; // temp unecessary import to cause warn so I know rust-analyzer parsed :)

//type wl_display = *mut core::wl_display;

/*/// Takes a known valid c str from the wayland compositor
/// and turns it into an owned String
///
/// Panics if any issue occurs during conversion
///
/// TODO: see if sealed type can be placed in ffi binds
/// that this can be considered "safe" for external use*/
/*pub unsafe fn from_cstr(input: *const std::os::raw::c_char) -> String {
    String::from(std::ffi::CStr::from_ptr(input).to_str().expect("Couldn't convert c string"))
}*/

impl Display {
    pub fn connect(name: Option<&str>) -> Result<Display, &str> {
        unsafe {
            let name = match name {
                    Some(string) => std::ffi::CString::new(string).map_err(|_| { "Couldn't convert display name"} )?.as_ptr(),
                    None => std::ptr::null(),
            };

            let ptr: *mut core::wl_display = core::wl_display_connect(name as *const std::os::raw::c_char);
            //println!("Maybe connected to display");

            //Some(Box::from_raw(ptr))
            match ptr.is_null() {
                true => Err("Display couldn't be found. Bad display name?"),
                false => Ok(Display { ptr }),
            }
        }
    }

    pub fn dispatch(&mut self) -> bool {
        unsafe {
            //println!("before call dispatch");
            let c_int_r: std::os::raw::c_int = core::wl_display_dispatch(self.ptr);
            //println!("after call dispatch");
            let native_r: i32 = c_int_r.into();

            native_r != -1
        }
    }

    pub fn get_registry(&mut self) -> Result<Registry, &str> {
        unsafe {
            //wl_registry { ptr: core::wl_display_get_registry(self.ptr) }
            //println!("inside get registry, get_registry const is {}", constants::WL_DISPLAY_GET_REGISTRY);
            let registry_ptr: *mut core::wl_proxy = core::wl_proxy_marshal_constructor(
                std::mem::transmute(self.ptr),
                constants::WL_DISPLAY_GET_REGISTRY,
                &core::wl_registry_interface,
                std::ptr::null::<*const std::ffi::c_void>()
            );

            //println!("called proxy marshal");

            match registry_ptr.is_null() {
                true => Err("Failed to get registry from given display handle"),
                false => Ok(Registry { ptr: std::mem::transmute(registry_ptr) }),
            }
        }
    }

    pub fn roundtrip(&mut self) {
        unsafe {
            println!("roundtripping");
            core::wl_display_roundtrip(self.ptr);
        }
    }

    /// Restrict passed data regions to impl Copy to require
    /// that destruction is trivial and non-virtual
    #[allow(dead_code)]
    pub unsafe fn add_listener<DisplayListenerData: Copy>(
        &mut self,
        onFatalError: unsafe extern "C" fn(
            data: Option<&mut DisplayListenerData>,
            wl_display: &mut core::wl_display,
            object_id: *mut std::os::raw::c_void,
            code: u32,
            *const std::os::raw::c_char),
        onObjectDelete: unsafe extern "C" fn(
            data: &mut DisplayListenerData,
            wl_display: &mut core::wl_display,
            id: u32),
        data: Box<DisplayListenerData>
    ) {
        let data_ptr = Box::into_raw(data);

        let listener = core::wl_display_listener {
            error: Some(std::mem::transmute(onFatalError)), delete_id: Some(std::mem::transmute(onObjectDelete))
        };

        let listener_ptr = Box::into_raw(Box::new(listener));

        core::wl_proxy_add_listener(
            self.ptr as *mut core::wl_proxy,
            std::mem::transmute(listener_ptr), // does cast from wl-client-core.h: 1038 to "void (**)(void)"
            std::mem::transmute::<*mut DisplayListenerData, *mut std::ffi::c_void>(data_ptr) // provides dld as void* for ffi
        );
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { 
            println!("dropping display...");
            core::wl_display_disconnect(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        println!("dropping registry...");
        // none yet
    }
}

/// Impl note: verify reentrant guards present on all functions where needed
unsafe impl Sync for Registry {}

impl Registry {
    pub fn from_raw(ptr: *mut core::wl_registry) -> Registry {
        Registry { ptr }
    }

    pub fn add_listener<RegistryListenerData>(
        &mut self,
        on_global: unsafe extern "C" fn(
            data: Option<&mut RegistryListenerData>,
            wl_registry: &mut core::wl_registry,
            name: u32,
            interface: *const std::os::raw::c_char,
            version: u32
        ),
        on_global_remove: unsafe extern "C" fn (
            data: Option<&mut RegistryListenerData>,
            wl_registry: &mut core::wl_registry,
            name: u32
        ),
        data: Option<Box<RegistryListenerData>>
    ) {
        unsafe {
            //println!("add listener called...");
            let data_ptr = match data {
                Some(data) => Box::into_raw(data),
                None => std::ptr::null(),
            };
            //println!("extracted data");

            let listener = core::wl_registry_listener {
                global: Some(std::mem::transmute(on_global)), global_remove: Some(std::mem::transmute(on_global_remove))
            };

            //println!("created listener");

            let listener_ptr = Box::into_raw(Box::new(listener));

            //println!("changed it to raw");

            let registry_ptr: *mut core::wl_registry = self.ptr;

            //let proxy_ptr: *mut core::wl_proxy = std::mem::transmute(registry_ptr);

            core::wl_proxy_add_listener(
                registry_ptr as *mut core::wl_proxy,
                std::mem::transmute(listener_ptr),
                std::mem::transmute(data_ptr)
            );

            //println!("added listener");
        }
    }

    /// Uses inferred expected type as interface descriminant
    ///
    /// Used to bind to an interface, equivalent to wl_registry_bind
    pub fn bind<T>(&mut self, name: u32, version: u32) -> Result<T, &str> where T: crate::types::FromWLInterface {
        unsafe {
        //let bind_string = T::get_type_str();
        //let cbind_string = std::ffi::CString::new(bind_string).expect("Couldn't parse ("as_ptr();
        /*let cbind_string = std::ffi::CString::new(bind_string)
            .map_err(|_| { "Couldn't parse interface string into valid cstring" })?
            .as_ptr();*/

            //let untyped_ptr = core::wl_registry_bind(self.ptr, name, T::get_interface(), version);
            let untyped_ptr = core::wl_proxy_marshal_constructor_versioned(
                self.ptr as *mut core::wl_proxy,
                constants::WL_REGISTRY_BIND,
                T::get_interface(),
                version,
                name,
                (*T::get_interface()).name,
                version,
                std::ptr::null::<*const std::ffi::c_void>()
            );


            T::from_void_handle(untyped_ptr as *mut std::ffi::c_void)
        }
    }
}

impl Compositor {
    pub fn create_surface(&mut self) -> Result<Surface, &str> {
        unsafe {
            let surface_ptr = core::wl_proxy_marshal_constructor(
                self.ptr as *mut core::wl_proxy,
                constants::WL_COMPOSITOR_CREATE_SURFACE,
                &core::wl_surface_interface,
                std::ptr::null::<*const std::ffi::c_void>()
            );

            match surface_ptr.is_null() {
                true => Err("Couldn't create surface"),
                false => Ok( Surface { ptr: surface_ptr as *mut core::wl_surface } ),
            }
        }
    }
}

impl Shm {
    pub fn add_listener<ShmListenerData>(
        &mut self,
        format: unsafe extern "C" fn(
            data: Option<&mut ShmListenerData>,
            wl_shm: &mut core::wl_shm,
            format: core::wl_shm_format
        ),
        data: Option<Box<ShmListenerData>>
    ) {
        unsafe {
            let data_ptr = match data {
                Some(data) => Box::into_raw(data),
                None => std::ptr::null(),
            };

            let listener = core::wl_shm_listener {
                format: Some(std::mem::transmute(format))
            };

            let listener_ptr = Box::into_raw(Box::new(listener));

            let shm_ptr: *mut core::wl_shm = self.ptr;

            core::wl_proxy_add_listener(
                shm_ptr as *mut core::wl_proxy,
                std::mem::transmute(listener_ptr),
                std::mem::transmute(data_ptr)
            );
        }
    }


}
