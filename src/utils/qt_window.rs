use raw_window_handle::{
    DisplayHandle, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    WindowHandle,
};

use crate::utils::error::WgpuError;

/// Owned display handle that wgpu keeps alive for the lifetime of the instance.
#[derive(Debug, Clone, Copy)]
pub struct QtDisplayHandle(RawDisplayHandle);

// SAFETY: the wrapped handle carries no borrowed data. On macOS and Windows it is empty, on Linux it
// is Xlib's "default display" (null pointer), so sharing it across threads is sound.
unsafe impl Send for QtDisplayHandle {}
unsafe impl Sync for QtDisplayHandle {}

impl HasDisplayHandle for QtDisplayHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, raw_window_handle::HandleError> {
        Ok(unsafe { DisplayHandle::borrow_raw(self.0) })
    }
}

/// Native window of a Qt widget (`QWidget.winId()`) in the form wgpu expects.
pub struct QtWindowHandle {
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
}

impl QtWindowHandle {
    /// `raw_window_handle` is the value of `QWidget.winId()`: `HWND` on Windows, `NSView*` on macOS
    /// and an X11 window id on Linux.
    pub fn new(raw_window_handle: usize) -> Result<Self, WgpuError> {
        if raw_window_handle == 0 {
            return Err(WgpuError::InvalidWindowHandle("winId() returned 0"));
        }
        let (window_handle, display_handle) = Self::from_native(raw_window_handle)?;
        Ok(Self {
            window_handle,
            display_handle,
        })
    }

    pub fn display(&self) -> QtDisplayHandle {
        QtDisplayHandle(self.display_handle)
    }

    #[cfg(target_os = "windows")]
    fn from_native(id: usize) -> Result<(RawWindowHandle, RawDisplayHandle), WgpuError> {
        use raw_window_handle::{Win32WindowHandle, WindowsDisplayHandle};
        let hwnd = std::num::NonZeroIsize::new(id as isize)
            .ok_or(WgpuError::InvalidWindowHandle("null HWND"))?;
        Ok((
            RawWindowHandle::Win32(Win32WindowHandle::new(hwnd)),
            RawDisplayHandle::Windows(WindowsDisplayHandle::new()),
        ))
    }

    #[cfg(target_os = "linux")]
    fn from_native(id: usize) -> Result<(RawWindowHandle, RawDisplayHandle), WgpuError> {
        use raw_window_handle::{XlibDisplayHandle, XlibWindowHandle};
        // Only X11 (and XWayland) is supported: a null display means "the default $DISPLAY".
        Ok((
            RawWindowHandle::Xlib(XlibWindowHandle::new(id as std::ffi::c_ulong)),
            RawDisplayHandle::Xlib(XlibDisplayHandle::new(None, 0)),
        ))
    }

    #[cfg(target_os = "macos")]
    fn from_native(id: usize) -> Result<(RawWindowHandle, RawDisplayHandle), WgpuError> {
        use raw_window_handle::{AppKitDisplayHandle, AppKitWindowHandle};
        let ns_view = std::ptr::NonNull::new(id as *mut std::ffi::c_void)
            .ok_or(WgpuError::InvalidWindowHandle("null NSView"))?;
        Ok((
            RawWindowHandle::AppKit(AppKitWindowHandle::new(ns_view)),
            RawDisplayHandle::AppKit(AppKitDisplayHandle::new()),
        ))
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    fn from_native(_id: usize) -> Result<(RawWindowHandle, RawDisplayHandle), WgpuError> {
        Err(WgpuError::UnsupportedPlatform)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_handle_is_rejected() {
        let err = QtWindowHandle::new(0).err().expect("0 must be rejected");
        assert!(matches!(err, WgpuError::InvalidWindowHandle(_)));
    }

    #[test]
    fn test_non_null_handle_is_accepted_and_never_dereferenced() {
        // The handle is only wrapped, wgpu dereferences it later, so a dummy value is fine here.
        let handle = QtWindowHandle::new(0x1000).expect("non-null handle");
        assert!(handle.window_handle().is_ok());
        assert!(handle.display_handle().is_ok());
    }

    #[test]
    fn test_display_handle_is_send_sync_static() {
        fn assert_send_sync<T: Send + Sync + 'static>() {}
        assert_send_sync::<QtDisplayHandle>();
    }
}
