use winapi::um::winuser::MessageBoxExW;

/// 参考 [WIN32文档](https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-messageboxexw)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
    Ok = 0x0,
    OkCancel = 0x1,
    AbortRetryIgnore = 0x2,
    YesNoCancel = 0x3,
    YesNo = 0x4,
    RetryCancel = 0x5,
    CancalRetryContinue = 0x6,
    Help = 0x4000,
}

/// 参考 [WIN32文档](https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-messageboxexw)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogType {
    Error = 0x10,
    Question = 0x20,
    Warning = 0x30,
    Information = 0x40,
}

/// 参考 [WIN32文档](https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-messageboxexw)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefaultButton {
    FirstButton = 0x0,
    SecondButton = 0x100,
    ThirdButton = 0x200,
    FourthButton = 0x300,
}

/// 参考 [WIN32文档](https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-messageboxexw)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modal {
    App = 0x0,
    System = 0x1000,
    Task = 0x2000,
}

/// 参考 [WIN32文档](https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-messageboxexw)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogOption {
    TopMost = 0x40000,
    Foreground = 0x10000,
    RightAlign = 0x80000,
    RtlReading = 0x100000,
    ServiceNotification = 0x200000,
    DefaultDesktopOnly = 0x20000,
}

/// 参考 [WIN32文档](https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-messageboxexw)
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogResult {
    Ok,
    Cancel,
    Abort,
    Retry,
    Ignore,
    Yes,
    No,
    RetryAgain,
    Continue,
}

/// 对话框
pub struct Dialog {
    hwnd: usize,
    title: String,
    message: String,
    utype: u32,
    lang_id: u16,
}

impl Dialog {
    /// 创建新的对话框
    pub fn new(type_: DialogType) -> Self {
        Self {
            hwnd: 0,
            title: "".into(),
            message: "".into(),
            utype: type_ as u32,
            lang_id: 0,
        }
    }

    /// 指示对话框标题栏内容
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    /// 指示对话框消息内容
    pub fn message(mut self, message: &str) -> Self {
        self.message = message.into();
        self
    }

    /// 指示对话框按钮类型
    pub fn buttons(mut self, buttons: Button) -> Self {
        self.utype |= buttons as u32;
        self
    }

    /// 指示对话框的默认按钮
    pub fn default_button(mut self, default_button: DefaultButton) -> Self {
        self.utype |= default_button as u32;
        self
    }

    /// 指示对话框的模式
    pub fn modal(mut self, modal: Modal) -> Self {
        self.utype |= modal as u32;
        self
    }

    /// 指示对话框的其它选项
    pub fn set(mut self, options: DialogOption) -> Self {
        self.utype |= options as u32;
        self
    }

    /// 激活对话框
    ///
    /// 该函数会阻塞当前线程，直到对话框产生用户反馈或被关闭
    ///
    /// # Return
    /// 返回用户的选择结果
    pub fn block(self) -> DialogResult {
        let title: Vec<_> = self.title.encode_utf16().chain(Some(0)).collect();
        let message: Vec<_> = self.message.encode_utf16().chain(Some(0)).collect();
        let result = unsafe {
            MessageBoxExW(
                self.hwnd as _,
                message.as_ptr(),
                title.as_ptr(),
                self.utype,
                self.lang_id,
            )
        };
        match result {
            1 => DialogResult::Ok,
            2 => DialogResult::Cancel,
            3 => DialogResult::Abort,
            4 => DialogResult::Retry,
            5 => DialogResult::Ignore,
            6 => DialogResult::Yes,
            7 => DialogResult::No,
            10 => DialogResult::RetryAgain,
            11 => DialogResult::Continue,
            _ => unreachable!(),
        }
    }
}
