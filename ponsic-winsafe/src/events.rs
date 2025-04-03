use winapi::shared::minwindef::{LPARAM, UINT, WPARAM};

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
pub enum WindowSize {
    Resize,
    Minimize,
    Maximize,
    Restore,
    MaxHide,
    MaxShow,
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
pub enum Event {
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
    Paint{
        context: Context,
    },
    Destroy,
    Create,
    Other {
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    },
}
