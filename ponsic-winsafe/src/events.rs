use winapi::{
    shared::minwindef::{LPARAM, UINT, WPARAM},
    um::winuser::*,
};

use crate::graphics::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,

    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    NumPad0,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Shift, // Shift
    Ctrl,  // Ctrl
    Alt,   // Alt

    // Special symbols for American keyboards, such as: ` , . / ; ' [ ] \ - =
    Symbol(char), // (Here use the symbol output when shift is not pressed to represent)

    NumAdd, // + on NumPad
    NumSub, // - on NumPad
    NumMul, // * on NumPad
    NumDiv, // / on NumPad
    NumDot, // . on NumPad

    Tab,       // Tab
    Space,     // Space
    Enter,     // Enter
    Backspace, // Backspace

    Esc,        // Esc
    CapsLock,   // CapsLock
    LeftCtrl,   // Left Ctrl
    LeftShift,  // Left Shift
    LeftAlt,    // Left Alt
    RightCtrl,  // Right Ctrl
    RightShift, // Right Shift
    RightAlt,   // Right Alt
    ScrollLock, // ScrollLock
    NumLock,    // NumLock
    Delete,     // Delete(Del)
    Insert,     // Insert(Ins)
    Home,       // Home
    End,        // End
    PageUp,     // PageUp(PgUp)
    PageDown,   // PageDown(PgDn)
    Clear,      // Clear(Num 5)

    LeftButton,   // Left mouse button
    RightButton,  // Right mouse button
    MiddleButton, // Middle mouse button
    X1Button,     // mouse extension button 1
    X2Button,     // mouse extension button 2

    Left,  // Left arrow
    Right, // Right arrow
    Up,    // Up arrow
    Down,  // Down arrow

    Unknown(i32), // Other keys
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
        key: KeyCode,
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
    Destroy,
    Create,
    UserDef {
        msg: u32,
        wparam: usize,
        lparam: isize,
    },
    Other {
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
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
    NoClient(NoClient),
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum NoClient {
    /// 回调函数处理此消息应该返回 None，以确保执行默认行为
    HitTest { x: i32, y: i32 },
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
