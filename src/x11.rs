/*
 * author: Aleksei Kozadaev (2022)
 */

#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]

use std::ffi::CString;
use std::ptr;

mod x11_ext {
    #[repr(C)]
    pub struct Window;
    pub type window_p = *const Window;

    pub struct Display;
    pub type display_p = *const Display;

    pub type cstr = *const i8;

    #[link(name = "X11")]
    extern "C" {
        pub fn XOpenDisplay(name: cstr) -> *mut display_p;
        pub fn XCloseDisplay(dpy: *mut display_p);
        pub fn XStoreName(dpy: *mut display_p, _: window_p, name: cstr);
        pub fn XDefaultRootWindow(dpy: *mut display_p) -> window_p;
        pub fn XSync(dpy: *mut display_p, discard: bool);
    }
}

fn open_display() -> *mut x11_ext::display_p {
    let dpy = unsafe { x11_ext::XOpenDisplay(ptr::null()) };
    if dpy.is_null() {
        panic!("failed to initialise screen")
    }

    dpy
}

fn close_display(dpy: *mut x11_ext::display_p) {
    unsafe {
        x11_ext::XCloseDisplay(dpy);
    }
}

pub struct Display {
    dpy: *mut x11_ext::display_p,
}

impl Display {
    pub fn new() -> Self {
        Display {
            dpy: open_display(),
        }
    }

    pub fn set_root_title(&mut self, title: String) {
        let title = CString::new(title).unwrap();

        unsafe {
            x11_ext::XStoreName(
                self.dpy,
                x11_ext::XDefaultRootWindow(self.dpy),
                title.as_ptr(),
            );
            x11_ext::XSync(self.dpy, true);
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        close_display(self.dpy)
    }
}
