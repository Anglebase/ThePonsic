use winapi::um::errhandlingapi::GetLastError;

use super::gen_by_py::translate_error;

#[derive(Debug)]
pub struct SystemError {
    pub code: u32,
    pub message: &'static str,
}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "系统错误: {}", self.message)
    }
}

impl std::error::Error for SystemError {}

impl SystemError {
    pub fn new(code: u32) -> Self {
        Self {
            code,
            message: translate_error(code),
        }
    }
}

/// 检查当前上下文中的错误代码
pub fn check_error() -> Result<(), SystemError> {
    let code = unsafe { GetLastError() };
    if code != 0 {
        return Err(SystemError::new(code));
    }
    Ok(())
}

/// 断言当前上下文中的没有发生错误
pub fn assert_no_error() {
    check_error().unwrap();
}