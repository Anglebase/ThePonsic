use crate::{The, WindowId, events::*, graphics::Context, win::window::WindowHandle};
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

const fn vk_to_key(vk: i32) -> KeyCode {
    match vk {
        0x30 => KeyCode::Num0,
        0x31 => KeyCode::Num1,
        0x32 => KeyCode::Num2,
        0x33 => KeyCode::Num3,
        0x34 => KeyCode::Num4,
        0x35 => KeyCode::Num5,
        0x36 => KeyCode::Num6,
        0x37 => KeyCode::Num7,
        0x38 => KeyCode::Num8,
        0x39 => KeyCode::Num9,

        0x41 => KeyCode::A,
        0x42 => KeyCode::B,
        0x43 => KeyCode::C,
        0x44 => KeyCode::D,
        0x45 => KeyCode::E,
        0x46 => KeyCode::F,
        0x47 => KeyCode::G,
        0x48 => KeyCode::H,
        0x49 => KeyCode::I,
        0x4A => KeyCode::J,
        0x4B => KeyCode::K,
        0x4C => KeyCode::L,
        0x4D => KeyCode::M,
        0x4E => KeyCode::N,
        0x4F => KeyCode::O,
        0x50 => KeyCode::P,
        0x51 => KeyCode::Q,
        0x52 => KeyCode::R,
        0x53 => KeyCode::S,
        0x54 => KeyCode::T,
        0x55 => KeyCode::U,
        0x56 => KeyCode::V,
        0x57 => KeyCode::W,
        0x58 => KeyCode::X,
        0x59 => KeyCode::Y,
        0x5A => KeyCode::Z,

        VK_F1 => KeyCode::F1,
        VK_F2 => KeyCode::F2,
        VK_F3 => KeyCode::F3,
        VK_F4 => KeyCode::F4,
        VK_F5 => KeyCode::F5,
        VK_F6 => KeyCode::F6,
        VK_F7 => KeyCode::F7,
        VK_F8 => KeyCode::F8,
        VK_F9 => KeyCode::F9,
        VK_F10 => KeyCode::F10,
        VK_F11 => KeyCode::F11,
        VK_F12 => KeyCode::F12,

        VK_NUMPAD0 => KeyCode::NumPad0,
        VK_NUMPAD1 => KeyCode::NumPad1,
        VK_NUMPAD2 => KeyCode::NumPad2,
        VK_NUMPAD3 => KeyCode::NumPad3,
        VK_NUMPAD4 => KeyCode::NumPad4,
        VK_NUMPAD5 => KeyCode::NumPad5,
        VK_NUMPAD6 => KeyCode::NumPad6,
        VK_NUMPAD7 => KeyCode::NumPad7,
        VK_NUMPAD8 => KeyCode::NumPad8,
        VK_NUMPAD9 => KeyCode::NumPad9,

        VK_SHIFT => KeyCode::Shift,
        VK_CONTROL => KeyCode::Ctrl,
        VK_MENU => KeyCode::Alt,

        VK_OEM_1 => KeyCode::Symbol(';'),
        VK_OEM_2 => KeyCode::Symbol('/'),
        VK_OEM_3 => KeyCode::Symbol('`'),
        VK_OEM_4 => KeyCode::Symbol('['),
        VK_OEM_5 => KeyCode::Symbol('\\'),
        VK_OEM_6 => KeyCode::Symbol(']'),
        VK_OEM_7 => KeyCode::Symbol('\''),
        VK_OEM_PLUS => KeyCode::Symbol('+'),
        VK_OEM_COMMA => KeyCode::Symbol(','),
        VK_OEM_MINUS => KeyCode::Symbol('-'),
        VK_OEM_PERIOD => KeyCode::Symbol('.'),

        VK_ADD => KeyCode::NumAdd,
        VK_SUBTRACT => KeyCode::NumSub,
        VK_MULTIPLY => KeyCode::NumMul,
        VK_DIVIDE => KeyCode::NumDiv,
        VK_DECIMAL => KeyCode::NumDot,

        VK_BACK => KeyCode::Backspace,
        VK_TAB => KeyCode::Tab,
        VK_RETURN => KeyCode::Enter,
        VK_SPACE => KeyCode::Space,

        VK_ESCAPE => KeyCode::Esc,
        VK_CAPITAL => KeyCode::CapsLock,
        VK_LCONTROL => KeyCode::LeftCtrl,
        VK_LSHIFT => KeyCode::LeftShift,
        VK_LMENU => KeyCode::LeftAlt,
        VK_RCONTROL => KeyCode::RightCtrl,
        VK_RSHIFT => KeyCode::RightShift,
        VK_RMENU => KeyCode::RightAlt,
        VK_SCROLL => KeyCode::ScrollLock,
        VK_NUMLOCK => KeyCode::NumLock,
        VK_DELETE => KeyCode::Delete,
        VK_INSERT => KeyCode::Insert,
        VK_HOME => KeyCode::Home,
        VK_END => KeyCode::End,
        VK_PRIOR => KeyCode::PageUp,
        VK_NEXT => KeyCode::PageDown,
        VK_CLEAR => KeyCode::Clear,

        VK_LBUTTON => KeyCode::LeftButton,
        VK_RBUTTON => KeyCode::RightButton,
        VK_MBUTTON => KeyCode::MiddleButton,
        VK_XBUTTON1 => KeyCode::X1Button,
        VK_XBUTTON2 => KeyCode::X2Button,

        VK_LEFT => KeyCode::Left,
        VK_UP => KeyCode::Up,
        VK_RIGHT => KeyCode::Right,
        VK_DOWN => KeyCode::Down,

        _ => KeyCode::Unknown(vk),
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
        WM_DESTROY => Event::Destroy,
        WM_CREATE => Event::Create,
        WM_PAINT => Event::Paint {
            context: unsafe { Context::from_raw(*hwnd) },
        },
        WM_GETMINMAXINFO => {
            let lparam = unsafe { (lparam as *mut MINMAXINFO).as_mut().unwrap() };
            Event::SizeRange {
                max_width: &mut lparam.ptMaxSize.x,
                max_height: &mut lparam.ptMaxSize.y,
                max_left: &mut lparam.ptMaxPosition.x,
                max_top: &mut lparam.ptMaxPosition.y,
                min_track_width: &mut lparam.ptMinTrackSize.x,
                min_track_height: &mut lparam.ptMinTrackSize.y,
                max_track_width: &mut lparam.ptMaxTrackSize.x,
                max_track_height: &mut lparam.ptMaxTrackSize.y,
            }
        }
        WM_SIZE => translate_window_size(wparam, lparam),
        WM_SIZING => {
            let lparam = unsafe { (lparam as *mut RECT).as_mut().unwrap() };
            Event::SizeChanging {
                ref_rect: RefRect {
                    left: &mut lparam.left,
                    top: &mut lparam.top,
                    right: &mut lparam.right,
                    bottom: &mut lparam.bottom,
                },
                type_: wparam_to_size_side(wparam),
            }
        }
        WM_NCHITTEST => translate_nc_hit_test(lparam),
        _ if msg >= WM_USER => Event::UserDef {
            msg: msg - WM_USER,
            wparam,
            lparam,
        },
        _ => Event::Other {
            msg,
            wparam,
            lparam,
        },
    }
}

#[allow(unused)]
const fn translate_nc_mouse_button(msg: UINT, wparam: WPARAM, lparam: LPARAM) -> Event<'static> {
    // let (x, y) = lparam_to_points(lparam);
    let (x, y) = (0, 0);
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
        pos: (x, y),
        status,
        at,
    })
}

#[allow(unused)]
const fn lparam_to_points(lparam: LPARAM) -> (i16, i16) {
    let p = unsafe { (lparam as *mut POINTS).as_ref().unwrap() };
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
    Event::SizeChanged {
        width,
        height,
        type_,
    }
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

/// 获取窗口所关联的数据
///
/// # Note
///
/// 不建议直接使用此方法
pub fn cast<T>(hwnd: WindowId) -> The<T> {
    let hwnd = unsafe { hwnd.handle() } as HWND;
    unsafe {
        let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
        The::from_raw(ptr as _)
    }
}

/// 此函数在宏 `wndproc!(...)` 中使用，不应直接调用
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
/// 它的参数应是`<type>;<fn>`的格式，`<type>`指示窗口关联数据的类型，若无关联数据，则以`()`占位
/// 传入的函数应符合 `impl Fn(Events) -> Option<isize>` (若无关联数据)
/// 或 `impl Fn(Events,The<T>) -> Option<isize>`(若有关联数据) 并且无外部捕获的非全局变量
/// + 若该函数返回 `None` 则将后续处理交由默认处理过程函数
/// + 若该函数返回 `Some(isize)` 则直接将内含值作为过程回调函数的返回值
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
                if let Some(__result) = __result {
                    __result
                } else {
                    $crate::default_proc(__hwnd, __msg, __wparam, __lparam)
                }
            }
            unsafe { $crate::WndProc::from_raw(__inner_wndproc) }
        }
    };
    ($t:ty ; $($f:tt)+) => {
        {
            extern "system" fn __inner_wndproc(__hwnd: $crate::HWND, __msg: u32, __wparam: usize, __lparam: isize) -> isize {
                if __msg == 0x2 /* WM_DESTROY */ {
                    unsafe { $crate::cast::<$t>($crate::WindowId::from_raw(__hwnd as _)).free() };
                }
                if __msg == 0x1 /* WM_CREATE */ {
                    $crate::bind_when_create(__hwnd, __lparam);
                }
                let __f = $($f)*;
                let __result = __f(
                    $crate::Events {
                        window: unsafe{ $crate::WindowHandle::from_raw(__hwnd) },
                        event: $crate::translate(&__hwnd, __msg, __wparam, __lparam),
                    },
                    $crate::cast::<$t>(unsafe { $crate::WindowId::from_raw(__hwnd as _) }),
                );
                if let Some(__result) = __result {
                    __result
                } else {
                    $crate::default_proc(__hwnd, __msg, __wparam, __lparam)
                }
            }
            unsafe { $crate::WndProc::from_raw(__inner_wndproc) }
        }
    };
}
