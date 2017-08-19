#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]
extern crate sdl2;

extern "C" {
    pub fn SDL_GL_GetProcAddress(name: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_void;
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
