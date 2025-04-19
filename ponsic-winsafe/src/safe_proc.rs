use crate::{events::*, graphics::Context, win::window::WindowHandle, TimerId};
pub use winapi::shared::windef::HWND;
use winapi::shared::{
    minwindef::{LPARAM, UINT, WPARAM},
    windef::{POINTS, RECT},
};
use winapi::um::winuser::*;

macro_rules! w_param_to_mod_key {
    ($w_param:expr) => {
        match $w_param {
            MK_LBUTTON => Some(ModifierKey::Mouse(Button::Left)),
            MK_RBUTTON => Some(ModifierKey::Mouse(Button::Right)),
            MK_MBUTTON => Some(ModifierKey::Mouse(Button::Middle)),
            MK_XBUTTON1 => Some(ModifierKey::Mouse(Button::X1)),
            MK_XBUTTON2 => Some(ModifierKey::Mouse(Button::X2)),
            MK_CONTROL => Some(ModifierKey::Ctrl),
            MK_SHIFT => Some(ModifierKey::Shift),
            _ => None,
        }
    };
}

#[inline]
const fn l_param_to_pos(l_param: LPARAM) -> (i32, i32) {
    ((l_param & 0xffff) as i32, ((l_param >> 16) & 0xffff) as i32)
}

const fn vk_to_key(vk: i32) -> Key {
    match vk {
        0x30 => Key::Num0,
        0x31 => Key::Num1,
        0x32 => Key::Num2,
        0x33 => Key::Num3,
        0x34 => Key::Num4,
        0x35 => Key::Num5,
        0x36 => Key::Num6,
        0x37 => Key::Num7,
        0x38 => Key::Num8,
        0x39 => Key::Num9,

        0x41 => Key::A,
        0x42 => Key::B,
        0x43 => Key::C,
        0x44 => Key::D,
        0x45 => Key::E,
        0x46 => Key::F,
        0x47 => Key::G,
        0x48 => Key::H,
        0x49 => Key::I,
        0x4A => Key::J,
        0x4B => Key::K,
        0x4C => Key::L,
        0x4D => Key::M,
        0x4E => Key::N,
        0x4F => Key::O,
        0x50 => Key::P,
        0x51 => Key::Q,
        0x52 => Key::R,
        0x53 => Key::S,
        0x54 => Key::T,
        0x55 => Key::U,
        0x56 => Key::V,
        0x57 => Key::W,
        0x58 => Key::X,
        0x59 => Key::Y,
        0x5A => Key::Z,

        VK_F1 => Key::F1,
        VK_F2 => Key::F2,
        VK_F3 => Key::F3,
        VK_F4 => Key::F4,
        VK_F5 => Key::F5,
        VK_F6 => Key::F6,
        VK_F7 => Key::F7,
        VK_F8 => Key::F8,
        VK_F9 => Key::F9,
        VK_F10 => Key::F10,
        VK_F11 => Key::F11,
        VK_F12 => Key::F12,

        VK_NUMPAD0 => Key::NumPad0,
        VK_NUMPAD1 => Key::NumPad1,
        VK_NUMPAD2 => Key::NumPad2,
        VK_NUMPAD3 => Key::NumPad3,
        VK_NUMPAD4 => Key::NumPad4,
        VK_NUMPAD5 => Key::NumPad5,
        VK_NUMPAD6 => Key::NumPad6,
        VK_NUMPAD7 => Key::NumPad7,
        VK_NUMPAD8 => Key::NumPad8,
        VK_NUMPAD9 => Key::NumPad9,

        VK_SHIFT => Key::Shift,
        VK_CONTROL => Key::Ctrl,
        VK_MENU => Key::Alt,

        VK_OEM_1 => Key::Semicolon,
        VK_OEM_2 => Key::Slash,
        VK_OEM_3 => Key::Backtick,
        VK_OEM_4 => Key::LeftBracket,
        VK_OEM_5 => Key::Backslash,
        VK_OEM_6 => Key::RightBracket,
        VK_OEM_7 => Key::Apostrophe,
        VK_OEM_PLUS => Key::Equals,
        VK_OEM_COMMA => Key::Comma,
        VK_OEM_MINUS => Key::Minus,
        VK_OEM_PERIOD => Key::Dot,

        VK_ADD => Key::NumAdd,
        VK_SUBTRACT => Key::NumSub,
        VK_MULTIPLY => Key::NumMul,
        VK_DIVIDE => Key::NumDiv,
        VK_DECIMAL => Key::NumDot,

        VK_BACK => Key::Backspace,
        VK_TAB => Key::Tab,
        VK_RETURN => Key::Enter,
        VK_SPACE => Key::Space,

        VK_ESCAPE => Key::Esc,
        VK_CAPITAL => Key::CapsLock,
        VK_LCONTROL => Key::LeftCtrl,
        VK_LSHIFT => Key::LeftShift,
        VK_LMENU => Key::LeftAlt,
        VK_RCONTROL => Key::RightCtrl,
        VK_RSHIFT => Key::RightShift,
        VK_RMENU => Key::RightAlt,
        VK_SCROLL => Key::ScrollLock,
        VK_NUMLOCK => Key::NumLock,
        VK_DELETE => Key::Delete,
        VK_INSERT => Key::Insert,
        VK_HOME => Key::Home,
        VK_END => Key::End,
        VK_PRIOR => Key::PageUp,
        VK_NEXT => Key::PageDown,
        VK_CLEAR => Key::Clear,

        VK_LBUTTON => Key::LeftButton,
        VK_RBUTTON => Key::RightButton,
        VK_MBUTTON => Key::MiddleButton,
        VK_XBUTTON1 => Key::X1Button,
        VK_XBUTTON2 => Key::X2Button,

        VK_LEFT => Key::Left,
        VK_UP => Key::Up,
        VK_RIGHT => Key::Right,
        VK_DOWN => Key::Down,

        _ => Key::Unknown(vk),
    }
}

/// 翻译窗口事件
///
/// # Note
/// 此函数由宏`wndproc!(...)`调用，不应直接调用
pub const fn translate(hwnd: &HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> Event {
    match msg {
        WM_LBUTTONDOWN | WM_LBUTTONUP | WM_LBUTTONDBLCLK | WM_MBUTTONDOWN | WM_MBUTTONUP
        | WM_MBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONUP | WM_RBUTTONDBLCLK | WM_XBUTTONDOWN
        | WM_XBUTTONUP | WM_XBUTTONDBLCLK => translate_mouse_button(msg, wparam, lparam),
        WM_NCLBUTTONDOWN | WM_NCLBUTTONUP | WM_NCLBUTTONDBLCLK | WM_NCMBUTTONDOWN
        | WM_NCMBUTTONUP | WM_NCMBUTTONDBLCLK | WM_NCRBUTTONDOWN | WM_NCRBUTTONUP
        | WM_NCRBUTTONDBLCLK | WM_NCXBUTTONDOWN | WM_NCXBUTTONUP | WM_NCXBUTTONDBLCLK => {
            translate_nc_mouse_button(msg, wparam, lparam)
        }
        WM_KEYDOWN | WM_KEYUP => translate_key_event(msg, wparam, lparam),
        WM_MOUSEMOVE => translate_mouse_move_event(wparam, lparam),
        WM_MOUSEWHEEL => translate_mouse_wheel_event(wparam, lparam),
        WM_CHAR => translate_text_input_event(wparam),
        WM_DESTROY => Event::Window(WindowEvent::Destroy),
        WM_CREATE => Event::Window(WindowEvent::Create),
        WM_NCMOUSELEAVE => Event::NoClient(NoClient::Leave),
        WM_NCCREATE => Event::NoClient(NoClient::Create),
        WM_CLOSE => Event::Window(WindowEvent::Close),
        WM_PAINT => Event::Paint {
            context: unsafe { Context::from_raw(*hwnd) },
        },
        WM_GETMINMAXINFO => {
            let lparam = unsafe { (lparam as *mut MINMAXINFO).as_mut().unwrap() };
            Event::Window(WindowEvent::SizeRange {
                max_width: &mut lparam.ptMaxSize.x,
                max_height: &mut lparam.ptMaxSize.y,
                max_left: &mut lparam.ptMaxPosition.x,
                max_top: &mut lparam.ptMaxPosition.y,
                min_track_width: &mut lparam.ptMinTrackSize.x,
                min_track_height: &mut lparam.ptMinTrackSize.y,
                max_track_width: &mut lparam.ptMaxTrackSize.x,
                max_track_height: &mut lparam.ptMaxTrackSize.y,
            })
        }
        WM_SIZE => translate_window_size(wparam, lparam),
        WM_SIZING => {
            let lparam = unsafe { (lparam as *mut RECT).as_mut().unwrap() };
            Event::Window(WindowEvent::SizeChanging {
                ref_rect: RefRect {
                    left: &mut lparam.left,
                    top: &mut lparam.top,
                    right: &mut lparam.right,
                    bottom: &mut lparam.bottom,
                },
                type_: wparam_to_size_side(wparam),
            })
        }
        WM_MOVE => translate_window_move(lparam),
        WM_NCHITTEST => translate_nc_hit_test(lparam),
        WM_NCMOUSEMOVE => translate_nc_mouse_move_event(wparam, lparam),
        WM_TIMER => Event::Timer {
            id: TimerId::new(wparam as _),
        },
        _ if msg >= WM_USER => Event::Window(WindowEvent::UserDef {
            msg,
            wparam,
            lparam,
        }),
        _ => Event::Other {
            msg,
            wparam,
            lparam,
        },
    }
}

const fn translate_window_move(lparam: LPARAM) -> Event<'static> {
    let pos = l_param_to_pos(lparam);
    Event::Window(WindowEvent::Move { pos })
}

const fn translate_nc_mouse_move_event(wparam: WPARAM, lparam: LPARAM) -> Event<'static> {
    let pos = lparam_to_points(lparam);
    let at = match (wparam & 0xffff) as isize {
        HTBORDER => CursorAt::Border,
        HTBOTTOM => CursorAt::Bottom,
        HTBOTTOMLEFT => CursorAt::BottomLeft,
        HTBOTTOMRIGHT => CursorAt::BottomRight,
        HTCAPTION => CursorAt::Caption,
        HTCLIENT => CursorAt::Client,
        HTCLOSE => CursorAt::Close,
        HTERROR => CursorAt::Error,
        HTHELP => CursorAt::Help,
        HTHSCROLL => CursorAt::HScroll,
        HTLEFT => CursorAt::Left,
        HTMENU => CursorAt::Menu,
        HTMAXBUTTON => CursorAt::MaxButton,
        HTMINBUTTON => CursorAt::MinButton,
        HTNOWHERE => CursorAt::NoWhere,
        HTRIGHT => CursorAt::Right,
        HTSIZE => CursorAt::Size,
        HTSYSMENU => CursorAt::Sysmenu,
        HTTOP => CursorAt::Top,
        HTTOPLEFT => CursorAt::TopLeft,
        HTTOPRIGHT => CursorAt::TopRight,
        HTTRANSPARENT => CursorAt::Transparent,
        HTVSCROLL => CursorAt::VScroll,
        _ => unreachable!(),
    };

    Event::NoClient(NoClient::Move { pos, at })
}

#[allow(unused)]
const fn translate_nc_mouse_button(msg: UINT, wparam: WPARAM, lparam: LPARAM) -> Event<'static> {
    let pos = lparam_to_points(lparam);
    let at = match (wparam & 0xffff) as isize {
        HTBORDER => CursorAt::Border,
        HTBOTTOM => CursorAt::Bottom,
        HTBOTTOMLEFT => CursorAt::BottomLeft,
        HTBOTTOMRIGHT => CursorAt::BottomRight,
        HTCAPTION => CursorAt::Caption,
        HTCLIENT => CursorAt::Client,
        HTCLOSE => CursorAt::Close,
        HTERROR => CursorAt::Error,
        HTHELP => CursorAt::Help,
        HTHSCROLL => CursorAt::HScroll,
        HTLEFT => CursorAt::Left,
        HTMENU => CursorAt::Menu,
        HTMAXBUTTON => CursorAt::MaxButton,
        HTMINBUTTON => CursorAt::MinButton,
        HTNOWHERE => CursorAt::NoWhere,
        HTRIGHT => CursorAt::Right,
        HTSIZE => CursorAt::Size,
        HTSYSMENU => CursorAt::Sysmenu,
        HTTOP => CursorAt::Top,
        HTTOPLEFT => CursorAt::TopLeft,
        HTTOPRIGHT => CursorAt::TopRight,
        HTTRANSPARENT => CursorAt::Transparent,
        HTVSCROLL => CursorAt::VScroll,
        _ => unreachable!(),
    };

    let button = match msg {
        WM_NCLBUTTONDOWN | WM_NCLBUTTONUP | WM_NCLBUTTONDBLCLK => Button::Left,
        WM_NCMBUTTONDOWN | WM_NCMBUTTONUP | WM_NCMBUTTONDBLCLK => Button::Middle,
        WM_NCRBUTTONDOWN | WM_NCRBUTTONUP | WM_NCRBUTTONDBLCLK => Button::Right,
        WM_NCXBUTTONDOWN | WM_NCXBUTTONUP | WM_NCXBUTTONDBLCLK => {
            match ((wparam >> 16) & 0xffff) as u16 {
                XBUTTON1 => Button::X1,
                XBUTTON2 => Button::X2,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };
    let status = match msg {
        WM_NCLBUTTONDOWN | WM_NCMBUTTONDOWN | WM_NCRBUTTONDOWN | WM_NCXBUTTONDOWN => {
            ButtonStatus::Down
        }
        WM_NCLBUTTONUP | WM_NCMBUTTONUP | WM_NCRBUTTONUP | WM_NCXBUTTONUP => ButtonStatus::Up,
        WM_NCLBUTTONDBLCLK | WM_NCMBUTTONDBLCLK | WM_NCRBUTTONDBLCLK | WM_NCXBUTTONDBLCLK => {
            ButtonStatus::DoubleClick
        }
        _ => unreachable!(),
    };
    Event::NoClient(NoClient::Mouse {
        button,
        pos,
        status,
        at,
    })
}

#[allow(unused)]
const fn lparam_to_points(lparam: LPARAM) -> (i16, i16) {
    let p = unsafe { (&lparam as *const _ as *const POINTS).as_ref().unwrap() };
    (p.x, p.y)
}

const fn translate_nc_hit_test(lparam: LPARAM) -> Event<'static> {
    let x = (lparam & 0xffff) as i32;
    let y = (lparam >> 16) as i32;
    Event::NoClient(NoClient::HitTest { x, y })
}

const fn wparam_to_size_side(wparam: WPARAM) -> SizingSide {
    match wparam {
        1 => SizingSide::Left,
        3 => SizingSide::Top,
        2 => SizingSide::Right,
        6 => SizingSide::Bottom,
        4 => SizingSide::TopLeft,
        5 => SizingSide::TopRight,
        7 => SizingSide::BottomLeft,
        8 => SizingSide::BottomRight,
        9 => SizingSide::MoveCauseExitMaximize,
        param @ _ => SizingSide::Unknown(param),
    }
}

const fn l_param_to_size(l_param: LPARAM) -> (u32, u32) {
    let width = (l_param >> 16) as u32;
    let height = (l_param & 0xffff) as u32;
    (width, height)
}

const fn translate_window_size(w_param: WPARAM, l_param: LPARAM) -> Event<'static> {
    let (width, height) = l_param_to_size(l_param);
    let type_ = match w_param {
        SIZE_MAXHIDE => SizeChangeType::MaxHide,
        SIZE_MAXIMIZED => SizeChangeType::Maximize,
        SIZE_MAXSHOW => SizeChangeType::MaxShow,
        SIZE_MINIMIZED => SizeChangeType::Minimize,
        SIZE_RESTORED => SizeChangeType::Restore,
        param @ _ => SizeChangeType::Unknown(param),
    };
    Event::Window(WindowEvent::SizeChanged {
        width,
        height,
        type_,
    })
}

const fn translate_mouse_button(msg: UINT, w_param: WPARAM, l_param: LPARAM) -> Event<'static> {
    let button = match msg {
        WM_LBUTTONDOWN | WM_LBUTTONUP | WM_LBUTTONDBLCLK => Button::Left,
        WM_MBUTTONDOWN | WM_MBUTTONUP | WM_MBUTTONDBLCLK => Button::Middle,
        WM_RBUTTONDOWN | WM_RBUTTONUP | WM_RBUTTONDBLCLK => Button::Right,
        WM_XBUTTONDOWN | WM_XBUTTONUP | WM_XBUTTONDBLCLK => {
            match ((w_param >> 16) & 0xffff) as u16 {
                XBUTTON1 => Button::X1,
                XBUTTON2 => Button::X2,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };
    let status = match msg {
        WM_LBUTTONDOWN | WM_MBUTTONDOWN | WM_RBUTTONDOWN | WM_XBUTTONDOWN => ButtonStatus::Down,
        WM_LBUTTONUP | WM_MBUTTONUP | WM_RBUTTONUP | WM_XBUTTONUP => ButtonStatus::Up,
        WM_LBUTTONDBLCLK | WM_MBUTTONDBLCLK | WM_RBUTTONDBLCLK | WM_XBUTTONDBLCLK => {
            ButtonStatus::DoubleClick
        }
        _ => unreachable!(),
    };
    let pos = l_param_to_pos(l_param);
    let modifier = w_param_to_mod_key!(w_param);
    Event::Mouse {
        button,
        pos,
        status,
        modifier,
    }
}

const fn translate_key_event(msg: UINT, w_param: WPARAM, l_param: LPARAM) -> Event<'static> {
    let status = if msg == WM_KEYDOWN {
        KeyStatus::Down
    } else {
        KeyStatus::Up
    };
    let key = vk_to_key(w_param as _);
    let ex_key = l_param & 0x00800000 == 0x00800000;
    Event::Key {
        key,
        ex_key,
        status,
    }
}

const fn translate_mouse_move_event(w_param: WPARAM, l_param: LPARAM) -> Event<'static> {
    let pos = l_param_to_pos(l_param);
    let modifier = w_param_to_mod_key!(w_param);
    Event::Move { pos, modifier }
}

const fn translate_mouse_wheel_event(w_param: WPARAM, l_param: LPARAM) -> Event<'static> {
    let pos = l_param_to_pos(l_param);
    let modifier = w_param_to_mod_key!(w_param & 0xffff);
    let wheel = if ((w_param >> 16) & 0xffff) as i16 > 0 {
        Wheel::Up
    } else {
        Wheel::Down
    };
    Event::Wheel {
        pos,
        wheel,
        modifier,
    }
}

/// 判断指定字节是否为UTF-16代理对的高 16 位
#[inline]
pub const fn is_high_surrogate(wch: u16) -> bool {
    wch >= 0xd800 && wch <= 0xdbff
}

/// 判断指定字节是否为UTF-16代理对的低 16 位
#[inline]
pub const fn is_low_surrogate(wch: u16) -> bool {
    wch >= 0xdc00 && wch <= 0xdfff
}

/// 将UTF-16代理对转换为UTF-32
#[inline]
pub const fn utf16_to_utf32(high: u16, low: u16) -> u32 {
    assert!(is_high_surrogate(high));
    assert!(is_low_surrogate(low));
    0x10000u32 + ((high - 0xd800) * 0x400) as u32 + (low - 0xdc00) as u32
}

const fn translate_text_input_event(w_param: WPARAM) -> Event<'static> {
    Event::Input { ch: w_param as _ }
}

/// 窗口默认行为函数
///
/// # Note
/// 此函数由宏`wndproc!(...)`调用，不应直接调用
pub fn default_proc(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> isize {
    return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
}

/// 窗口过程
///
/// # Note
/// 此结构体应由宏`wndproc!(...)`创建
pub struct WndProc {
    proc: extern "system" fn(HWND, u32, usize, isize) -> isize,
}

impl WndProc {
    pub unsafe fn from_raw(proc: extern "system" fn(HWND, u32, usize, isize) -> isize) -> Self {
        Self { proc }
    }

    pub unsafe fn into_raw(self) -> extern "system" fn(HWND, u32, usize, isize) -> isize {
        self.proc
    }
}

pub struct Events<'a> {
    pub window: WindowHandle,
    pub event: Event<'a>,
}

/// 此函数在宏 `wndproc!(...)` 中使用，不应直接调用
#[deprecated(since = "0.1.0", note = "不应显式调用此方法")]
pub fn bind_when_create(hwnd: HWND, lparam: isize) {
    let lparam = lparam as *const CREATESTRUCTW;
    unsafe {
        if let Some(lparam) = lparam.as_ref() {
            let ptr = lparam.lpCreateParams;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr as _);
        }
    }
}

/// 生成窗口处理过程函数宏
///
/// # Param
/// 它的参数应是`<type>;<fn>`的格式，`<type>`指示窗口关联数据的类型，若无关联数据，则以`()`占位；
/// 传入的函数应符合 `impl Fn(Events) -> Return` (若无关联数据)
/// 或 `impl Fn(Events,The<T>) -> Return`(若有关联数据) 并且无外部捕获的非全局变量
///
/// # Result
/// 它生成的值是 `WndProc` 对象，可直接作为`&Class.set_process()`函数的参数
#[macro_export]
macro_rules! wndproc {
    (() ; $($f:tt)+) => {
        {
            extern "system" fn __inner_wndproc(__hwnd: $crate::HWND, __msg: u32, __wparam: usize, __lparam: isize) -> isize {
                let __f = $($f)*;
                let __result = __f(
                    $crate::Events {
                        window: unsafe{ $crate::WindowHandle::from_raw(__hwnd) },
                        event: $crate::translate(&__hwnd, __msg, __wparam, __lparam),
                    }
                );
                match __result {
                    $crate::Return::Finish => 0,
                    $crate::Return::Default => $crate::default_proc(__hwnd, __msg, __wparam, __lparam),
                    $crate::Return::Data(data) => data,
                }
            }
            unsafe { $crate::WndProc::from_raw(__inner_wndproc) }
        }
    };
    ($t:ty ; $($f:tt)+) => {
        {
            extern "system" fn __inner_wndproc(__hwnd: $crate::HWND, __msg: u32, __wparam: usize, __lparam: isize) -> isize {
                if __msg == 0x81 /* WM_CREATE */ {
                    #[allow(deprecated)]
                    $crate::bind_when_create(__hwnd, __lparam);
                }
                let __f = $($f)*;
                let __result = __f(
                    $crate::Events {
                        window: unsafe{ $crate::WindowHandle::from_raw(__hwnd) },
                        event: $crate::translate(&__hwnd, __msg, __wparam, __lparam),
                    },
                    #[allow(deprecated)]
                    unsafe { $crate::assert_cast::<$t>($crate::WindowId::from_raw(__hwnd as _)) },
                );
                if __msg == 0x82 /* WM_DESTROY */ {
                    #[allow(deprecated)]
                    unsafe { $crate::cast_warpper_and_free::<$t>(
                        $crate::WindowId::from_raw(__hwnd as _)
                    ) };
                }
                match __result {
                    $crate::Return::Finish => 0,
                    $crate::Return::Default => $crate::default_proc(__hwnd, __msg, __wparam, __lparam),
                    $crate::Return::Data(data) => data,
                }
            }
            unsafe { $crate::WndProc::from_raw(__inner_wndproc) }
        }
    };
}
