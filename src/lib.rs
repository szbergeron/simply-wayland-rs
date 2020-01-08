#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod helpers;

pub mod wl {

    pub mod core {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
        pub use crate::helpers::*;
    }

    //pub mod extra 

    /*pub mod helper {
        pub use crate::helpers::*;
    }*/

    pub mod constants {
        include!(concat!(env!("OUT_DIR"), "/constants.rs"));
    }
}
