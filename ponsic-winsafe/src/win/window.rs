use std::any::Any;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::ptr::{null, null_mut};
use winapi::shared::minwindef::BOOL;
use winapi::shared::windef::*;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winnt::{HANDLE, LPCWSTR};
use winapi::um::winuser::*;

/// 参考 [WIN32 窗口样式](https://learn.microsoft.com/zh-cn/windows/win32/winmsg/window-styles)
/// 及 [WIN32 扩展窗口样式](https://learn.microsoft.com/zh-cn/windows/win32/winmsg/extended-window-styles)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WindowStyle {
    AcceptFiles,
    AppWindow,
    ClientEdge,
    Composited,
    ContextHelp,
    ControlParent,
    DlgModalFrame,
    Layered,
    LayoutRtl,
    Left,
    LeftScrollBar,
    LtrReading,
    MdiChild,
    NoActivate,
    NoInheritLayout,
    NoParentNotify,
    NoRedirectionBitmap,
    ExOverlappedWindow,
    WindowEdge,
    PaletteWindow,
    ToolWindow,
    TopMost,
    Right,
    RightScrollBar,
    RtlReading,
    StaticEdge,
    Transparent,

    ChildWindow,
    ClipChildren,
    ClipSiblings,
    Disabled,
    DlgFrame,
    Group,
    HScroll,
    Maximize,
    Minimize,
    Iconic,
    Child,
    Popup,
    Border,
    PopupWindow,
    TabStop,
    SizeBox,
    Tiled,
    TiledWindow,
    Overlapped,
    Caption,
    SysMenu,
    ThickFrame,
    MinimizeBox,
    MaximizeBox,
    OverlappedWindow,
    Visible,
    VScroll,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Window {
    handle: HWND,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WindowHandle {
    pub(crate) handle: HWND,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WindowId {
    handle: usize,
}

#[derive(Debug)]
pub struct WindowError {
    message: String,
    code: u32,
}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[WindowError {}: {}]", self.code, self.message)
    }
}

impl std::error::Error for WindowError {}

impl Window {
    fn new(handle: HWND) -> Self {
        Self { handle }
    }

    pub fn handle(&self) -> WindowHandle {
        WindowHandle {
            handle: self.handle,
        }
    }
}

pub trait WindowManager {
    fn get_handle(&self) -> usize;

    fn id(&self) -> WindowId {
        WindowId {
            handle: self.get_handle() as _,
        }
    }

    fn show(&self) {
        let handle = self.get_handle() as HWND;
        unsafe {
            ShowWindow(handle, SW_SHOW);
            UpdateWindow(handle);
        }
    }

    /// 设置窗口文本
    ///
    /// # Note
    /// 此函数通过向回调函数发送请求信息来设置窗口文本，不应在回调函数中无条件调用，否则会引发无限递归而导致栈溢出
    fn set_title(&self, title: &str) {
        let handle = self.get_handle() as HWND;
        let title: Vec<u16> = String::from(title).encode_utf16().chain(Some(0)).collect();
        unsafe {
            SetWindowTextW(handle, title.as_ptr());
        }
    }

    /// 获取窗口文本
    ///
    /// # Note
    /// 此函数通过向回调函数发送请求信息来获取窗口文本，不应在回调函数中无条件调用，否则会引发无限递归而导致栈溢出
    fn title(&self) -> String {
        let handle = self.get_handle() as HWND;
        unsafe {
            let len = GetWindowTextLengthW(handle) as usize + 1;
            let mut title: Vec<u16> = vec![0; len];
            GetWindowTextW(handle, title.as_mut_ptr(), len as i32);
            String::from_utf16(&title[0..len - 1]).unwrap()
        }
    }

    /// 设置窗口的父窗口
    fn set_parent(&self, parent: WindowId) {
        let handle = self.get_handle() as HWND;
        let parent = parent.handle as HWND;
        unsafe {
            SetParent(handle, parent);
        }
    }

    fn parent(&self) -> Option<WindowHandle> {
        let handle = self.get_handle() as HWND;
        unsafe {
            let parent = GetParent(handle);
            if parent.is_null() {
                None
            } else {
                Some(WindowHandle { handle })
            }
        }
    }

    fn insert<T: 'static>(&self, name: &str, value: T) -> Option<Box<dyn Any>> {
        let name: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        let mut result = None;
        unsafe {
            let prop: HANDLE = GetPropW(self.get_handle() as _, name.as_ptr());
            if !prop.is_null() {
                result = Some(Box::from_raw(prop as *mut Box<dyn Any>));
            }
            let value: Box<Box<dyn Any>> = Box::new(Box::new(value));
            let prop = Box::into_raw(value);
            SetPropW(self.get_handle() as _, name.as_ptr(), prop as _);
        }
        result.map(|v| v as Box<dyn Any>)
    }

    fn get<'a>(&'a self, name: &str) -> Option<&'a dyn Any> {
        let name: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        unsafe {
            let prop: HANDLE = GetPropW(self.get_handle() as _, name.as_ptr());
            let prop = prop as *mut Box<dyn Any>;
            if let Some(val) = prop.as_ref() {
                Some(val.as_ref())
            } else {
                None
            }
        }
    }

    fn get_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut dyn Any> {
        let name: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        unsafe {
            let prop: HANDLE = GetPropW(self.get_handle() as _, name.as_ptr());
            let prop = prop as *mut Box<dyn Any>;
            if let Some(val) = prop.as_mut() {
                Some(val.as_mut())
            } else {
                None
            }
        }
    }

    fn remove(&mut self, name: &str) -> Option<Box<dyn Any>> {
        let name: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        unsafe {
            let prop: HANDLE = RemovePropW(self.get_handle() as _, name.as_ptr());
            if !prop.is_null() {
                Some(Box::from_raw(prop as *mut Box<dyn Any>) as Box<dyn Any>)
            } else {
                None
            }
        }
    }

    fn redraw(&self) {
        unsafe {
            RedrawWindow(
                self.get_handle() as _,
                null_mut(),
                null_mut(),
                RDW_UPDATENOW | RDW_INVALIDATE,
            );
        }
    }
}

impl Window {
    /// 向指定窗口回调函数发送自定义消息
    ///
    /// # Note
    /// 此函数的调用是同步的，它将等待回调函数对消息进行处理，并返回回调函数的返回值
    pub fn send(window: WindowId, msg: u32, wparam: usize, lparam: isize) -> isize {
        unsafe { SendMessageW(window.handle as _, WM_USER + msg, wparam, lparam) }
    }

    /// 向指定窗口回调函数发送自定义消息
    ///
    /// # Note
    /// 此函数的调用是异步的，它不等待回调函数对消息进行处理，直接返回，此函数不应传递引用
    pub fn post(window: WindowId, msg: u32, wparam: usize, lparam: isize) -> Result<i32, u32> {
        match unsafe { PostMessageW(window.handle as _, msg + WM_USER, wparam, lparam) } {
            0 => {
                let err = unsafe { GetLastError() };
                Err(err)
            }
            res @ _ => Ok(res),
        }
    }

    pub const USER_DEF_BASE: u32 = WM_USER;

    pub unsafe fn send_not_userdef(
        window: WindowId,
        msg: u32,
        wparam: usize,
        lparam: isize,
    ) -> isize {
        unsafe { SendMessageW(window.handle as _, msg, wparam, lparam) }
    }

    pub unsafe fn post_not_userdef(
        window: WindowId,
        msg: u32,
        wparam: usize,
        lparam: isize,
    ) -> Result<i32, u32> {
        match unsafe { PostMessageW(window.handle as _, msg, wparam, lparam) } {
            0 => {
                let err = unsafe { GetLastError() };
                Err(err)
            }
            res @ _ => Ok(res),
        }
    }
}

impl WindowManager for Window {
    fn get_handle(&self) -> usize {
        self.handle as usize
    }
}

impl WindowManager for WindowHandle {
    fn get_handle(&self) -> usize {
        self.handle as usize
    }
}

impl Index<&str> for Window {
    type Output = dyn Any;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<&str> for Window {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            EnumPropsW(self.handle, Some(enum_prop));
        }
    }
}

unsafe extern "system" fn enum_prop(handle: HWND, prop: LPCWSTR, _: HANDLE) -> BOOL {
    let data = unsafe { RemovePropW(handle, prop) };
    if !data.is_null() {
        let _ = unsafe { Box::from_raw(data as *mut Box<dyn Any>) };
        true as _
    } else {
        false as _
    }
}

#[derive(Debug)]
pub struct Builder {
    pos_size: (i32, i32, i32, i32),
    class_name: String,
    extra_styles: u32,
    style: u32,
    title: String,
    parent: Option<WindowId>,
}

impl Builder {
    pub fn set_parent(mut self, window: WindowId) -> Self {
        self.parent = Some(window);
        self
    }
    pub(super) fn new(class_name: &str, pos_size: (i32, i32, i32, i32)) -> Self {
        Self {
            pos_size,
            class_name: class_name.into(),
            extra_styles: 0,
            style: 0,
            title: "Window".into(),
            parent: None,
        }
    }

    pub fn set_style(mut self, style: &[WindowStyle]) -> Self {
        for &style in style {
            match style {
                WindowStyle::AcceptFiles => {
                    self.extra_styles |= WS_EX_ACCEPTFILES;
                }
                WindowStyle::AppWindow => {
                    self.extra_styles |= WS_EX_APPWINDOW;
                }
                WindowStyle::ClientEdge => {
                    self.extra_styles |= WS_EX_CLIENTEDGE;
                }
                WindowStyle::Composited => {
                    self.extra_styles |= WS_EX_COMPOSITED;
                }
                WindowStyle::Transparent => {
                    self.extra_styles |= WS_EX_TRANSPARENT;
                }
                WindowStyle::ContextHelp => {
                    self.extra_styles |= WS_EX_CONTEXTHELP;
                }
                WindowStyle::ControlParent => {
                    self.extra_styles |= WS_EX_CONTROLPARENT;
                }
                WindowStyle::DlgModalFrame => {
                    self.extra_styles |= WS_EX_DLGMODALFRAME;
                }
                WindowStyle::Layered => {
                    self.extra_styles |= WS_EX_LAYERED;
                }
                WindowStyle::LayoutRtl => {
                    self.extra_styles |= WS_EX_LAYOUTRTL;
                }
                WindowStyle::Left => {
                    self.extra_styles |= WS_EX_LEFT;
                }
                WindowStyle::LeftScrollBar => {
                    self.extra_styles |= WS_EX_LEFTSCROLLBAR;
                }
                WindowStyle::LtrReading => {
                    self.extra_styles |= WS_EX_LTRREADING;
                }
                WindowStyle::MdiChild => {
                    self.extra_styles |= WS_EX_MDICHILD;
                }
                WindowStyle::NoActivate => {
                    self.extra_styles |= WS_EX_NOACTIVATE;
                }
                WindowStyle::NoInheritLayout => {
                    self.extra_styles |= WS_EX_NOINHERITLAYOUT;
                }
                WindowStyle::NoParentNotify => {
                    self.extra_styles |= WS_EX_NOPARENTNOTIFY;
                }
                WindowStyle::NoRedirectionBitmap => {
                    self.extra_styles |= WS_EX_NOREDIRECTIONBITMAP;
                }
                WindowStyle::ExOverlappedWindow => {
                    self.extra_styles |= WS_EX_OVERLAPPEDWINDOW;
                }
                WindowStyle::WindowEdge => {
                    self.extra_styles |= WS_EX_WINDOWEDGE;
                }
                WindowStyle::PaletteWindow => {
                    self.extra_styles |= WS_EX_PALETTEWINDOW;
                }
                WindowStyle::ToolWindow => {
                    self.extra_styles |= WS_EX_TOOLWINDOW;
                }
                WindowStyle::TopMost => {
                    self.extra_styles |= WS_EX_TOPMOST;
                }
                WindowStyle::Right => {
                    self.extra_styles |= WS_EX_RIGHT;
                }
                WindowStyle::RightScrollBar => {
                    self.extra_styles |= WS_EX_RIGHTSCROLLBAR;
                }
                WindowStyle::RtlReading => {
                    self.extra_styles |= WS_EX_RTLREADING;
                }
                WindowStyle::StaticEdge => {
                    self.extra_styles |= WS_EX_STATICEDGE;
                }
                WindowStyle::ChildWindow => {
                    self.style |= WS_CHILDWINDOW;
                }
                WindowStyle::ClipChildren => {
                    self.style |= WS_CLIPCHILDREN;
                }
                WindowStyle::ClipSiblings => {
                    self.style |= WS_CLIPSIBLINGS;
                }
                WindowStyle::Disabled => {
                    self.style |= WS_DISABLED;
                }
                WindowStyle::DlgFrame => {
                    self.style |= WS_DLGFRAME;
                }
                WindowStyle::Group => {
                    self.style |= WS_GROUP;
                }
                WindowStyle::HScroll => {
                    self.style |= WS_HSCROLL;
                }
                WindowStyle::Maximize => {
                    self.style |= WS_MAXIMIZE;
                }
                WindowStyle::Minimize => {
                    self.style |= WS_MINIMIZE;
                }
                WindowStyle::Iconic => {
                    self.style |= WS_ICONIC;
                }
                WindowStyle::Child => {
                    self.style |= WS_CHILD;
                }
                WindowStyle::Popup => {
                    self.style |= WS_POPUP;
                }
                WindowStyle::Border => {
                    self.style |= WS_BORDER;
                }
                WindowStyle::PopupWindow => {
                    self.style |= WS_POPUPWINDOW;
                }
                WindowStyle::TabStop => {
                    self.style |= WS_TABSTOP;
                }
                WindowStyle::SizeBox => {
                    self.style |= WS_SIZEBOX;
                }
                WindowStyle::Tiled => {
                    self.style |= WS_TILED;
                }
                WindowStyle::TiledWindow => {
                    self.style |= WS_TILEDWINDOW;
                }
                WindowStyle::Overlapped => {
                    self.style |= WS_OVERLAPPED;
                }
                WindowStyle::Caption => {
                    self.style |= WS_CAPTION;
                }
                WindowStyle::SysMenu => {
                    self.style |= WS_SYSMENU;
                }
                WindowStyle::ThickFrame => {
                    self.style |= WS_THICKFRAME;
                }
                WindowStyle::MinimizeBox => {
                    self.style |= WS_MINIMIZEBOX;
                }
                WindowStyle::MaximizeBox => {
                    self.style |= WS_MAXIMIZEBOX;
                }
                WindowStyle::OverlappedWindow => {
                    self.style |= WS_OVERLAPPEDWINDOW;
                }
                WindowStyle::Visible => {
                    self.style |= WS_VISIBLE;
                }
                WindowStyle::VScroll => {
                    self.style |= WS_VSCROLL;
                }
            }
        }
        self
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    pub fn build(self) -> super::Result<Window> {
        let class_name: Vec<u16> = self.class_name.encode_utf16().chain(Some(0)).collect();
        let title: Vec<u16> = self.title.encode_utf16().chain(Some(0)).collect();
        let handle = unsafe {
            let handle = CreateWindowExW(
                self.extra_styles,
                class_name.as_ptr(),
                title.as_ptr(),
                self.style,
                self.pos_size.0,
                self.pos_size.1,
                self.pos_size.2,
                self.pos_size.3,
                if let Some(window) = self.parent {
                    window.handle as HWND
                } else {
                    null_mut()
                },
                null_mut(),
                GetModuleHandleW(null()),
                null_mut(),
            );
            if handle.is_null() {
                let error_code = GetLastError();
                let error_message = format!("Failed to Create Window '{}'", self.title);
                let error = WindowError {
                    message: error_message,
                    code: error_code,
                };
                return Err(Box::new(error));
            }
            handle
        };
        Ok(Window::new(handle))
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Result, *};
    use crate::win::class;

    #[test]
    fn window_builder_test() -> Result<()> {
        let class = class::Registrar::new("window_builder_test").build()?;

        let window = class
            .window_builder(100, 100, 800, 600)
            .set_title("Test")
            .set_style(&[WindowStyle::OverlappedWindow, WindowStyle::Border]);

        #[allow(unused_variables)]
        let window = window.build()?;

        Ok(())
    }

    #[test]
    fn window_prop_test() -> Result<()> {
        let class = class::Registrar::new("window_prop_test").build()?;

        let window = class
            .window_builder(100, 100, 800, 600)
            .set_title("Test")
            .set_style(&[WindowStyle::OverlappedWindow, WindowStyle::Border])
            .build()?;
        window.insert("UserData", 42);
        window.insert("Hello", "World");

        let v1 = window.get("UserData").unwrap();
        let v2 = window.get("Hello").unwrap();

        assert_eq!(*v1.downcast_ref::<i32>().unwrap(), 42);
        assert_eq!(*v2.downcast_ref::<&str>().unwrap(), "World");

        Ok(())
    }
}

impl WindowHandle {
    pub unsafe fn from_raw(handle: HWND) -> Self {
        WindowHandle { handle }
    }
}
