use ponsic_types::Recti;
use std::fmt::Debug;
use std::ptr::{null, null_mut};
use winapi::shared::windef::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;

use crate::{SystemError, check_error, make_ptr};

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

/// 窗口实例
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Window {
    handle: HWND,
}

/// 窗口句柄
///
/// 若要在线程间传递窗口标识，应使用 `WindowId`
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WindowHandle {
    pub(crate) handle: HWND,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WindowId {
    handle: usize,
}

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
    fn set_text(&self, title: &str) {
        let handle = self.get_handle() as HWND;
        let title: Vec<u16> = String::from(title).encode_utf16().chain(Some(0)).collect();
        unsafe {
            SetWindowTextW(handle, title.as_ptr());
        }
    }

    /// 设置窗口文本
    ///
    /// # Note
    /// 此函数通过向回调函数发送请求信息来设置窗口文本，不应在回调函数中无条件调用，否则会引发无限递归而导致栈溢出
    #[deprecated(
        since = "1.0.0",
        note = "此方法已弃用，请使用 WindowManager::set_text() 代替"
    )]
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
    fn text(&self) -> String {
        let handle = self.get_handle() as HWND;
        unsafe {
            let len = GetWindowTextLengthW(handle) as usize + 1;
            let mut title: Vec<u16> = vec![0; len];
            GetWindowTextW(handle, title.as_mut_ptr(), len as i32);
            String::from_utf16(&title[0..len - 1]).unwrap()
        }
    }

    /// 获取窗口文本
    ///
    /// # Note
    /// 此函数通过向回调函数发送请求信息来获取窗口文本，不应在回调函数中无条件调用，否则会引发无限递归而导致栈溢出
    #[deprecated(
        since = "1.0.0",
        note = "此方法已弃用，请使用 WindowManager::text() 代替"
    )]
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

    /// 获取窗口父窗口的句柄
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

    /// 指示窗口应该被重新绘制
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

    fn get_rect(&self) -> Recti {
        let mut rect = unsafe { std::mem::zeroed::<RECT>() };
        unsafe { GetWindowRect(self.get_handle() as _, &mut rect) };
        Recti::new(rect.left, rect.top, rect.right, rect.bottom)
    }

    fn get_client_rect(&self) -> Recti {
        let mut rect = unsafe { std::mem::zeroed::<RECT>() };
        unsafe { GetClientRect(self.get_handle() as _, &mut rect) };
        Recti::new(rect.left, rect.top, rect.right, rect.bottom)
    }
}

impl Window {
    /// 向指定窗口回调函数发送自定义消息
    ///
    /// # Note
    /// 此函数的调用是同步的，它将等待回调函数对消息进行处理，并返回回调函数的返回值
    pub unsafe fn send(window: WindowId, msg: u32, wparam: usize, lparam: isize) -> isize {
        unsafe { SendMessageW(window.handle as _, msg, wparam, lparam) }
    }

    /// 向指定窗口回调函数发送自定义消息
    ///
    /// # Note
    /// 此函数的调用是异步的，它不等待回调函数对消息进行处理，直接返回，此函数不应传递引用
    pub unsafe fn post(
        window: WindowId,
        msg: u32,
        wparam: usize,
        lparam: isize,
    ) -> Result<i32, SystemError> {
        match unsafe { PostMessageW(window.handle as _, msg, wparam, lparam) } {
            0 => Err(check_error().unwrap_err()),
            res @ _ => Ok(res),
        }
    }

    /// 用户自定义消息的起始值
    pub const USER_DEF_BASE: u32 = WM_USER;
    pub const APP_DEF_BASE: u32 = WM_APP;
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

/// 窗口构建器
#[derive(Debug)]
pub struct Builder {
    pos_size: (i32, i32, u32, u32),
    class_name: String,
    extra_styles: u32,
    style: u32,
    title: String,
    parent: Option<WindowId>,
    ptr: usize,
}

impl Builder {
    pub(super) fn new(class_name: &str, pos_size: (i32, i32, u32, u32)) -> Self {
        Self {
            pos_size,
            class_name: class_name.into(),
            extra_styles: 0,
            style: 0,
            title: "Window".into(),
            parent: None,
            ptr: 0,
        }
    }

    /// 设置窗口的父窗口
    pub fn set_parent(mut self, window: WindowId) -> Self {
        self.parent = Some(window);
        self
    }

    /// 设置窗口的样式
    ///
    /// 参考 [WindowStyle]
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

    /// 设置窗口标题
    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    /// 绑定窗口的关联数据
    pub fn bind_data<T>(mut self, data: T) -> Self {
        self.ptr = make_ptr(data) as _;
        self
    }

    /// 创建窗口
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
                self.pos_size.2 as _,
                self.pos_size.3 as _,
                if let Some(window) = self.parent {
                    window.handle as HWND
                } else {
                    null_mut()
                },
                null_mut(),
                GetModuleHandleW(null()),
                self.ptr as _,
            );
            if handle.is_null() {
                check_error()?;
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
    use ponsic_types::{Point, Recti as Rect, Size};

    #[test]
    fn window_builder_test() -> Result<()> {
        let class = class::Registrar::new("window_builder_test").build()?;

        let window = class
            .make_window(Rect::from((Point::new(100, 100), Size::new(800, 600))))
            .set_title("Test")
            .set_style(&[WindowStyle::OverlappedWindow, WindowStyle::Border]);

        #[allow(unused_variables)]
        let window = window.build()?;

        Ok(())
    }
}

impl WindowHandle {
    pub unsafe fn from_raw(handle: HWND) -> Self {
        WindowHandle { handle }
    }
}

impl WindowId {
    pub unsafe fn handle(&self) -> usize {
        self.handle
    }

    pub unsafe fn from_raw(handle: usize) -> Self {
        WindowId { handle }
    }
}
