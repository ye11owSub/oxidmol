use raw_window_handle::{
    AppKitDisplayHandle, AppKitWindowHandle, DisplayHandle, HasDisplayHandle, HasWindowHandle,
    RawDisplayHandle, RawWindowHandle, Win32WindowHandle, WindowHandle, WindowsDisplayHandle,
    XlibDisplayHandle, XlibWindowHandle,
};
use std::{ffi::c_ulong, num::NonZeroIsize, ptr::NonNull};

pub struct QtWindowHandle {
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
}

impl QtWindowHandle {
    pub fn new(raw_window_handle: usize) -> Self {
        let error_message = "Window handle is null";

        let (window_handle, display_handle) = match () {
            _ if cfg!(target_os = "windows") => {
                let win32_handle = Win32WindowHandle::new(
                    NonZeroIsize::new(raw_window_handle as isize).expect(error_message),
                );
                let display_handle = WindowsDisplayHandle::new();
                (
                    RawWindowHandle::Win32(win32_handle),
                    RawDisplayHandle::Windows(display_handle),
                )
            }
            _ if cfg!(target_os = "linux") => {
                let xlib_handle = XlibWindowHandle::new(raw_window_handle as c_ulong);
                let display_handle = XlibDisplayHandle::new(None, 0);
                (
                    RawWindowHandle::Xlib(xlib_handle),
                    RawDisplayHandle::Xlib(display_handle),
                )
            }
            _ if cfg!(target_os = "macos") => {
                let appkit_handle = AppKitWindowHandle::new(
                    NonNull::new(raw_window_handle as *mut std::ffi::c_void).expect(error_message),
                );
                let display_handle = AppKitDisplayHandle::new();

                (
                    RawWindowHandle::AppKit(appkit_handle),
                    RawDisplayHandle::AppKit(display_handle),
                )
            }
            _ => panic!("Unsupported operating system"),
        };

        Self {
            window_handle,
            display_handle,
        }
    }
}

impl HasWindowHandle for QtWindowHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, raw_window_handle::HandleError> {
        Ok(unsafe { WindowHandle::borrow_raw(self.window_handle) })
    }
}

impl HasDisplayHandle for QtWindowHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, raw_window_handle::HandleError> {
        Ok(unsafe { DisplayHandle::borrow_raw(self.display_handle) })
    }
}
