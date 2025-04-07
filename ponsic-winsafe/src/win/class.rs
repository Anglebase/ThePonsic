use crate::check_error;
use crate::graphics::context_2d::Rect;
use crate::safe_proc::WndProc;
use crate::win::window;
use std::ptr::null_mut;
use winapi::shared::windef::{HBRUSH, HCURSOR};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;

/// 参考 [WIN32 类样式](https://learn.microsoft.com/zh-cn/windows/win32/winmsg/window-class-styles)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ClassStyle {
    ByteAlignClient,
    ByteAlignWindow,
    UnionClassDeviceContext,
    AcceptDoubleClicks,
    DropShadow,
    GlobalClass,
    HorizontalRedraw,
    NoClose,
    OwnerDeviceContext,
    ParentDeviceContext,
    SaveBits,
    VerticalRedraw,
}

/// 参考 [WIN32 预定义光标](https://learn.microsoft.com/zh-cn/windows/win32/menurc/about-cursors)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Cursor {
    Arrow,
    TextSelect,
    Wait,
    Cross,
    UpArrow,
    SizeNorthWestSouthEast,
    SizeNorthEastSouthWest,
    SizeWestEast,
    SizeNorthSouth,
    SizeAll,
    No,
    Hand,
    AppStarting,
    Help,
    Pin,
    Person,
}

/// 窗口类构建器
#[derive(Debug, Clone)]
pub struct ClassError {
    message: String,
}

impl std::fmt::Display for ClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ClassError: {}]", self.message)
    }
}

impl std::error::Error for ClassError {}

/// 窗口类
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Class {
    name: String,
}

impl Class {
    fn new(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn make_window(&self, rect: Rect) -> window::Builder {
        window::Builder::new(
            &self.name,
            (rect.x(), rect.y(), rect.width(), rect.height()),
        )
    }
}

/// 参考 [WIN32 预定义控件类](https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-createwindowexw#remarks)
pub struct PreDefineClass;

impl PreDefineClass {
    pub fn button() -> Class {
        Class::new("BUTTON")
    }

    pub fn combobox() -> Class {
        Class::new("COMBOBOX")
    }

    pub fn edit() -> Class {
        Class::new("EDIT")
    }

    pub fn listbox() -> Class {
        Class::new("LISTBOX")
    }

    pub fn mdi_client() -> Class {
        Class::new("MDICLIENT")
    }

    pub fn rich_edit() -> Class {
        Class::new("RichEdit")
    }

    pub fn rich_edit_class() -> Class {
        Class::new("RICHEDIT_CLASS")
    }

    pub fn scrollbar() -> Class {
        Class::new("SCROLLBAR")
    }

    pub fn static_widget() -> Class {
        Class::new("STATIC")
    }
}

/// 窗口类注册器
#[derive(Debug)]
pub struct Registrar {
    style: u32,
    process: WNDPROC,
    name: String,
    cursor: HCURSOR,
}

impl Registrar {
    pub fn new(name: &str) -> Self {
        Self {
            style: 0,
            process: None,
            name: name.into(),
            cursor: null_mut(),
        }
    }

    pub fn set_style(mut self, styles: &[ClassStyle]) -> Self {
        for &style in styles {
            self.style |= match style {
                ClassStyle::ByteAlignClient => CS_BYTEALIGNCLIENT,
                ClassStyle::ByteAlignWindow => CS_BYTEALIGNWINDOW,
                ClassStyle::UnionClassDeviceContext => CS_CLASSDC,
                ClassStyle::AcceptDoubleClicks => CS_DBLCLKS,
                ClassStyle::DropShadow => CS_DROPSHADOW,
                ClassStyle::GlobalClass => CS_GLOBALCLASS,
                ClassStyle::HorizontalRedraw => CS_HREDRAW,
                ClassStyle::NoClose => CS_NOCLOSE,
                ClassStyle::OwnerDeviceContext => CS_OWNDC,
                ClassStyle::ParentDeviceContext => CS_PARENTDC,
                ClassStyle::SaveBits => CS_SAVEBITS,
                ClassStyle::VerticalRedraw => CS_VREDRAW,
            };
        }
        self
    }

    /// 设置窗口处理函数
    ///
    /// 参考函数式宏 `wndproc!(...)`
    pub fn set_process(mut self, process: WndProc) -> Self {
        self.process = Some(unsafe { process.into_raw() });
        self
    }

    pub fn set_cursor(mut self, cursor: Cursor) -> Self {
        let id = match cursor {
            Cursor::Arrow => IDC_ARROW,
            Cursor::TextSelect => IDC_IBEAM,
            Cursor::Wait => IDC_WAIT,
            Cursor::Cross => IDC_CROSS,
            Cursor::UpArrow => IDC_UPARROW,
            Cursor::SizeNorthWestSouthEast => IDC_SIZENWSE,
            Cursor::SizeNorthEastSouthWest => IDC_SIZENESW,
            Cursor::SizeWestEast => IDC_SIZEWE,
            Cursor::SizeNorthSouth => IDC_SIZENS,
            Cursor::SizeAll => IDC_SIZEALL,
            Cursor::No => IDC_NO,
            Cursor::Hand => IDC_HAND,
            Cursor::AppStarting => IDC_APPSTARTING,
            Cursor::Help => IDC_HELP,
            Cursor::Pin => MAKEINTRESOURCEW(32671),
            Cursor::Person => MAKEINTRESOURCEW(32672),
        };
        self.cursor = unsafe { LoadCursorW(null_mut(), id) };
        self
    }

    pub fn build(self) -> super::Result<Class> {
        let class_name: Vec<u16> = self.name.clone().encode_utf16().chain(Some(0)).collect();
        unsafe {
            let window_class_msg: WNDCLASSEXW = WNDCLASSEXW {
                cbSize: size_of::<WNDCLASSEXW>() as u32,
                style: self.style,
                lpfnWndProc: Some(self.process.unwrap_or(DefWindowProcW)),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: GetModuleHandleW(null_mut()),
                hIcon: null_mut(),
                hCursor: self.cursor,
                hbrBackground: COLOR_WINDOW as HBRUSH,
                lpszMenuName: null_mut(),
                lpszClassName: class_name.as_ptr() as _,
                hIconSm: null_mut(),
            };
            let result = RegisterClassExW(&window_class_msg);
            if result == 0 {
                check_error()?;
            }
        }
        Ok(Class::new(&self.name))
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Result, *};

    #[test]
    fn builder_test() -> Result<()> {
        let _ = Registrar::new("Test")
            .set_style(&[ClassStyle::AcceptDoubleClicks])
            .build()?;
        Ok(())
    }
}
