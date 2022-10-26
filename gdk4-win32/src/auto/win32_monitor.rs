// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkWin32Monitor")]
    pub struct Win32Monitor(Object<ffi::GdkWin32Monitor, ffi::GdkWin32MonitorClass>) @extends gdk::Monitor;

    match fn {
        type_ => || ffi::gdk_win32_monitor_get_type(),
    }
}

impl Win32Monitor {
    #[doc(alias = "gdk_win32_monitor_get_workarea")]
    #[doc(alias = "get_workarea")]
    pub fn workarea(monitor: &impl IsA<gdk::Monitor>) -> gdk::Rectangle {
        assert_initialized_main_thread!();
        unsafe {
            let mut workarea = gdk::Rectangle::uninitialized();
            ffi::gdk_win32_monitor_get_workarea(
                monitor.as_ref().to_glib_none().0,
                workarea.to_glib_none_mut().0,
            );
            workarea
        }
    }
}

impl fmt::Display for Win32Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Win32Monitor")
    }
}
