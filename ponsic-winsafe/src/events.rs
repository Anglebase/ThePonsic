use winapi::{
    shared::minwindef::{LPARAM, UINT, WPARAM},
    um::winuser::*,
};

use crate::{TimerId, graphics::Context};

/// 按键标识
///
/// 此枚举标识(美式)键盘上所有的按键
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    /// A
    A,
    /// B
    B,
    /// C
    C,
    /// D
    D,
    /// E
    E,
    /// F
    F,
    /// G
    G,
    /// H
    H,
    /// I
    I,
    /// J
    J,
    /// K
    K,
    /// L
    L,
    /// M
    M,
    /// N
    N,
    /// O
    O,
    /// P
    P,
    /// Q
    Q,
    /// R
    R,
    /// S
    S,
    /// T
    T,
    /// U
    U,
    /// V
    V,
    /// W
    W,
    /// X
    X,
    /// Y
    Y,
    /// Z
    Z,

    /// 1
    Num1,
    /// 2
    Num2,
    /// 3
    Num3,
    /// 4
    Num4,
    /// 5
    Num5,
    /// 6
    Num6,
    /// 7
    Num7,
    /// 8
    Num8,
    /// 9
    Num9,
    /// 0
    Num0,

    /// NumPad 1
    NumPad1,
    /// NumPad 2
    NumPad2,
    /// NumPad 3
    NumPad3,
    /// NumPad 4
    NumPad4,
    /// NumPad 5
    NumPad5,
    /// NumPad 6
    NumPad6,
    /// NumPad 7
    NumPad7,
    /// NumPad 8
    NumPad8,
    /// NumPad 9
    NumPad9,
    /// NumPad 0
    NumPad0,

    /// F1
    F1,
    /// F2
    F2,
    /// F3
    F3,
    /// F4
    F4,
    /// F5
    F5,
    /// F6
    F6,
    /// F7
    F7,
    /// F8
    F8,
    /// F9
    F9,
    /// F10
    F10,
    /// F11
    F11,
    /// F12
    F12,

    /// Shift
    Shift,
    /// Ctrl
    Ctrl,
    /// Alt
    Alt,

    // Special symbols for American keyboards
    /// `
    Backtick,
    /// ,
    Comma,
    /// .
    Dot,
    /// /
    Slash,
    /// ;
    Semicolon,
    /// '
    Apostrophe,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// \
    Backslash,
    /// -
    Minus,
    /// =
    Equals,

    /// NumPad +
    NumAdd,
    /// NumPad -
    NumSub,
    /// NumPad *
    NumMul,
    /// NumPad /
    NumDiv,
    /// NumPad .
    NumDot,

    /// Tab
    Tab,
    /// Space
    Space,
    /// Enter
    Enter,
    /// Backspace
    Backspace,

    /// Esc
    Esc,
    /// CapsLock
    CapsLock,
    /// Left Ctrl
    LeftCtrl,
    /// Left Shift
    LeftShift,
    /// Left Alt
    LeftAlt,
    /// Right Ctrl
    RightCtrl,
    /// Right Shift
    RightShift,
    /// Right Alt
    RightAlt,
    /// ScrollLock
    ScrollLock,
    /// NumLock
    NumLock,
    /// Delete
    Delete,
    /// Insert
    Insert,
    /// Home
    Home,
    /// End
    End,
    /// PageUp
    PageUp,
    /// PageDown
    PageDown,
    /// Clear
    Clear,

    /// Left mouse button
    LeftButton,
    /// Right mouse button
    RightButton,
    /// Middle mouse button
    MiddleButton,
    /// Mouse extension button 1
    X1Button,
    /// Mouse extension button 2
    X2Button,

    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,

    /// Unknown key
    Unknown(i32),
}

impl PartialEq<char> for Key {
    #[inline(never)]
    fn eq(&self, other: &char) -> bool {
        match (self, other) {
            // 字母键 (A-Z)
            (Key::A, 'a' | 'A') => true,
            (Key::B, 'b' | 'B') => true,
            (Key::C, 'c' | 'C') => true,
            (Key::D, 'd' | 'D') => true,
            (Key::E, 'e' | 'E') => true,
            (Key::F, 'f' | 'F') => true,
            (Key::G, 'g' | 'G') => true,
            (Key::H, 'h' | 'H') => true,
            (Key::I, 'i' | 'I') => true,
            (Key::J, 'j' | 'J') => true,
            (Key::K, 'k' | 'K') => true,
            (Key::L, 'l' | 'L') => true,
            (Key::M, 'm' | 'M') => true,
            (Key::N, 'n' | 'N') => true,
            (Key::O, 'o' | 'O') => true,
            (Key::P, 'p' | 'P') => true,
            (Key::Q, 'q' | 'Q') => true,
            (Key::R, 'r' | 'R') => true,
            (Key::S, 's' | 'S') => true,
            (Key::T, 't' | 'T') => true,
            (Key::U, 'u' | 'U') => true,
            (Key::V, 'v' | 'V') => true,
            (Key::W, 'w' | 'W') => true,
            (Key::X, 'x' | 'X') => true,
            (Key::Y, 'y' | 'Y') => true,
            (Key::Z, 'z' | 'Z') => true,

            // 数字键 (0-9)
            (Key::Num0 | Key::NumPad0, '0') => true,
            (Key::Num1 | Key::NumPad1, '1') => true,
            (Key::Num2 | Key::NumPad2, '2') => true,
            (Key::Num3 | Key::NumPad3, '3') => true,
            (Key::Num4 | Key::NumPad4, '4') => true,
            (Key::Num5 | Key::NumPad5, '5') => true,
            (Key::Num6 | Key::NumPad6, '6') => true,
            (Key::Num7 | Key::NumPad7, '7') => true,
            (Key::Num8 | Key::NumPad8, '8') => true,
            (Key::Num9 | Key::NumPad9, '9') => true,

            // Shift + 数字键
            (Key::Num0, ')') => true,
            (Key::Num1, '!') => true,
            (Key::Num2, '@') => true,
            (Key::Num3, '#') => true,
            (Key::Num4, '$') => true,
            (Key::Num5, '%') => true,
            (Key::Num6, '^') => true,
            (Key::Num7, '&') => true,
            (Key::Num8, '*') => true,
            (Key::Num9, '(') => true,

            // 符号键
            (Key::Backtick, '`' | '~') => true,
            (Key::Comma, ',' | '<') => true,
            (Key::Dot, '.' | '>') => true,
            (Key::Slash, '/' | '?') => true,
            (Key::Semicolon, ';' | ':') => true,
            (Key::Apostrophe, '\'' | '"') => true,
            (Key::LeftBracket, '[' | '{') => true,
            (Key::RightBracket, ']' | '}') => true,
            (Key::Backslash, '\\' | '|') => true,
            (Key::Minus, '-' | '_') => true,
            (Key::Equals, '=' | '+') => true,

            // 小键盘符号
            (Key::NumAdd, '+') => true,
            (Key::NumSub, '-') => true,
            (Key::NumMul, '*') => true,
            (Key::NumDiv, '/') => true,
            (Key::NumDot, '.') => true,

            // 控制字符
            (Key::Space, ' ') => true,
            (Key::Tab, '\t') => true,
            (Key::Enter, '\n' | '\r') => true, // 同时支持 LF 和 CR
            (Key::Backspace, '\x08') => true,  // ASCII 退格符

            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Button {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Wheel {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ModifierKey {
    Shift,
    Ctrl,
    Alt,
    Win,
    Mouse(Button),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum SizeChangeType {
    Resize,
    Minimize,
    Maximize,
    Restore,
    MaxHide,
    MaxShow,
    Unknown(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct HotKeyFlags {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub win: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ButtonStatus {
    Down,
    Up,
    DoubleClick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum KeyStatus {
    Down,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum SizingSide {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    MoveCauseExitMaximize,
    Unknown(usize),
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct RefRect<'a> {
    pub left: &'a mut i32,
    pub top: &'a mut i32,
    pub right: &'a mut i32,
    pub bottom: &'a mut i32,
}

#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum CursorAt {
    Border = HTBORDER,
    Bottom = HTBOTTOM,
    BottomLeft = HTBOTTOMLEFT,
    BottomRight = HTBOTTOMRIGHT,
    Caption = HTCAPTION,
    Client = HTCLIENT,
    Close = HTCLOSE,
    Error = HTERROR,
    Help = HTHELP,
    HScroll = HTHSCROLL,
    Left = HTLEFT,
    Menu = HTMENU,
    MaxButton = HTMAXBUTTON,
    MinButton = HTMINBUTTON,
    NoWhere = HTNOWHERE,
    Right = HTRIGHT,
    Size = HTSIZE,
    Sysmenu = HTSYSMENU,
    Top = HTTOP,
    TopLeft = HTTOPLEFT,
    TopRight = HTTOPRIGHT,
    Transparent = HTTRANSPARENT,
    VScroll = HTVSCROLL,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Event<'a> {
    Key {
        key: Key,
        ex_key: bool,
        status: KeyStatus,
    },
    Mouse {
        button: Button,
        pos: (i32, i32),
        status: ButtonStatus,
        modifier: Option<ModifierKey>,
    },
    Move {
        pos: (i32, i32),
        modifier: Option<ModifierKey>,
    },
    Wheel {
        pos: (i32, i32),
        wheel: Wheel,
        modifier: Option<ModifierKey>,
    },
    Input {
        ch: u16,
    },
    Paint {
        context: Context,
    },
    Timer {
        id: TimerId,
    },
    NoClient(NoClient),
    Window(WindowEvent<'a>),
    Other {
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    },
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum WindowEvent<'a> {
    Destroy,
    Create,
    Close,
    Move {
        pos: (i32, i32),
    },
    UserDef {
        msg: u32,
        wparam: usize,
        lparam: isize,
    },
    SizeRange {
        max_width: &'a mut i32,
        max_height: &'a mut i32,
        max_left: &'a mut i32,
        max_top: &'a mut i32,
        min_track_width: &'a mut i32,
        min_track_height: &'a mut i32,
        max_track_width: &'a mut i32,
        max_track_height: &'a mut i32,
    },
    SizeChanged {
        width: u32,
        height: u32,
        type_: SizeChangeType,
    },
    SizeChanging {
        ref_rect: RefRect<'a>,
        type_: SizingSide,
    },
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum NoClient {
    /// 回调函数处理此消息应该返回 None，以确保执行默认行为
    HitTest {
        x: i32,
        y: i32,
    },
    Mouse {
        button: Button,
        pos: (i16, i16),
        status: ButtonStatus,
        at: CursorAt,
    },
    Move {
        pos: (i16, i16),
        at: CursorAt,
    },
    Leave,
    Create,
}

/// 窗口过程函数返回值
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Return {
    /// 指示处理已完成且要返回 0
    Finish,
    /// 指示后续处理应交由窗口的默认行为
    Default,
    /// 指示处理已完成，但要返回自定义值
    Data(isize),
}
