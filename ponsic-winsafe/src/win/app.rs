use std::ptr::null_mut;
use winapi::shared::windef::POINT;
use winapi::um::winuser::*;

pub struct App;

impl App {
    /// 处理当前线程中所持有窗口产生的消息队列
    pub fn handle_events() {
        let mut msg: MSG = MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        unsafe {
            loop {
                let result = GetMessageW(&mut msg, null_mut(), 0, 0);
                if result == 0 {
                    break;
                } else {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }

    /// 通知当前线程所运行的`handle_events`事件循环退出
    pub fn should_exit(code: i32) {
        unsafe {
            PostQuitMessage(code);
        }
    }
}
