use std::ptr::null_mut;
use winapi::um::winuser::*;

pub struct App;

impl App {
    /// 处理当前线程中事件队列中的一个事件
    ///
    /// # Param
    /// - block: 是否阻塞等待事件
    ///     + 如果为`true`：
    ///         - 若事件队列不为空，处理事件队列中的第一个事件，然后返回；
    ///         - 若事件队列为空，将阻塞直到事件队列中有事件，然后处理事件队列中的第一个事件并返回
    ///     + 如果为`false`：
    ///         - 若事件队列不为空，处理事件队列中的第一个事件，然后返回；
    ///         - 若事件队列为空，则直接返回
    ///
    /// # Return
    /// - 若函数收到并处理一般事件，则返回`Some(true)`
    /// - 如函数收到并处理了请求退出的事件，则返回`Some(false)`
    /// - 若函数未处理任何有效事件，则返回`None`
    pub fn handle_event(block: bool) -> Option<bool> {
        let mut msg = unsafe { std::mem::zeroed::<MSG>() };
        let result = if block {
            let result = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
            Some(result != 0)
        } else {
            if let 0 = unsafe { PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) } {
                return None;
            }
            Some(msg.message != WM_QUIT)
        };
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        result
    }

    /// 向当前线程发出退出请求
    ///
    /// # Param
    /// - code: 退出代码
    /// # Note
    /// 此函数向调用线程发出退出请求，若当前线程中的`App::handle_event`处理该消息将返回`false`
    pub fn should_exit(code: i32) {
        unsafe {
            PostQuitMessage(code);
        }
    }
}
