use crate::wl::core;
use crate::wl::constants;

#[allow(non_camel_case_types)]
pub struct Display {
    ptr: *mut core::wl_display,
}

//type wl_display = *mut core::wl_display;

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

    pub fn get_registry(&mut self) -> Registry {
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

            Registry {
                ptr: std::mem::transmute(registry_ptr)
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
            std::mem::transmute::<*mut core::wl_display, *mut core::wl_proxy>(self.ptr),
            std::mem::transmute::<*mut core::wl_display_listener, _>(listener_ptr),
            std::mem::transmute::<*mut DisplayListenerData, *mut std::ffi::c_void>(data_ptr)
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

/*impl std::ops::Deref for wl_display {
    type Target = core::wl_display;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &self.ptr.as_ref().unwrap()
        }
    }
}*/

#[allow(non_camel_case_types)]
pub struct Registry {
    ptr: *mut core::wl_registry,
}

impl Drop for Registry {
    fn drop(&mut self) {
        println!("dropping registry...");
        // none yet
    }
}

impl Registry {
    pub fn add_listener<RegistryListenerData: Copy>(
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

            let proxy_ptr: *mut core::wl_proxy = std::mem::transmute(registry_ptr);

            core::wl_proxy_add_listener(
                proxy_ptr,
                std::mem::transmute(listener_ptr),
                std::mem::transmute(data_ptr)
            );

            //println!("added listener");
        }
    }
}

/*pub fn registry_handle_global(
    data: *mut std::os::raw::c_void,
    registry: *mut core::wl_registry,
    name: u32,
    interface: *const std::os::raw::c_char,
    version: u32,
) {
    unsafe {
        //println!("got global: {}", interface.to_str().unwrap());
        println!("got global: {} with name {}", std::ffi::CStr::from_ptr(interface).to_str().unwrap(), name);
    }
}

pub fn registry_handle_global_remove(
    data: *mut std::ffi::c_void,
    registry: *mut core::wl_registry,
    name: u32
) {
    println!("was asked to delete global {}", name);
}*/

/*pub fn bind_registry() {
    //
}*/

//pub fn sw_bind_global_listeners()

/*static core::wl_registry_listener {
    registry_handle_global,
    registry_handle_global_remove,
}*/

/*pub fn wl_display_connect(name: Option<&str>) -> Option<Box<core::wl_display>> {
    unsafe {
        let name = match name {
                Some(string) => std::ffi::CString::new(string).expect("Couldn't convert display name").as_ptr(),
                None => std::ptr::null(),
        };

        let ptr: *mut core::wl_display = core::wl_display_connect(name as *const std::os::raw::c_char);

        //Some(Box::from_raw(ptr))
        match ptr.is_null() {
            true => None,
            false => Some(Box::from_raw(ptr)),
        }
    }

}

pub fn wl_display_disconnect(display: Box<core::wl_display>) {
    unsafe {
        core::wl_display_disconnect(display.as_mut() as *mut core::wl_display);
    }
}*/

// False if error occurs
/*pub fn wl_display_dispatch() -> bool {
    unsafe {
        let c_int_r: std::os::raw::c_int = core::wl_display_dispatch(display as *mut core::wl_display);
        let native_r: i32 = c_int_r.into();

        native_r != -1
    }
}*/

/*fn cast_proxy_generic(proxy: *mut Proxy) -> *mut core::wl_proxy {
    unsafe {
        let as_generic: *mut core::wl_proxy = std::mem::transmute(proxy);
        as_generic
    }
}*/
