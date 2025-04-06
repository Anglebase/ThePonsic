use winapi::{
    shared::windef::HWND,
    um::winuser::{GWLP_USERDATA, GetWindowLongPtrW, SetWindowLongPtrW},
};

use crate::{The, WindowId};

/// 窗口绑定数据
pub struct WindowBindData<T> {
    pub data_ptr: *mut T,
    pub type_name: &'static str,
}

impl<T> WindowBindData<T> {
    pub(crate) fn new(data: T) -> Self {
        let data_ptr = Box::into_raw(Box::new(data));
        let type_name = std::any::type_name::<T>();
        Self {
            data_ptr,
            type_name,
        }
    }

    /// 释放窗口绑定数据
    ///
    /// # Note
    /// 此函数在宏`wndproc!(...)`中使用
    pub fn free(&self) {
        assert_eq!(self.type_name, std::any::type_name::<T>());
        unsafe {
            drop(Box::from_raw(self.data_ptr));
        }
    }
}

pub(crate) fn make_ptr<T>(data: T) -> *mut WindowBindData<T> {
    Box::into_raw(Box::new(WindowBindData::new(data)))
}

/// 获取窗口所关联的数据实例
///
/// # Note
/// 此函数在宏`wndproc!(...)`中使用
pub unsafe fn cast_warpper_and_free<T>(id: WindowId) {
    let ptr =
        unsafe { GetWindowLongPtrW(id.handle() as HWND, GWLP_USERDATA) as *mut WindowBindData<T> };
    if !ptr.is_null() {
        let warpper = *unsafe { Box::from_raw(ptr) };
        warpper.free();
        unsafe {
            SetWindowLongPtrW(id.handle() as HWND, GWLP_USERDATA, 0);
        }
    }
}

/// 获取窗口所关联的数据
///
/// # Safety
/// 使用此方法时需要保证其泛型类型与窗口所绑定的数据类型一致，否则会引发不可预料的后果
pub fn assert_cast<T>(hwnd: WindowId) -> The<T> {
    let hwnd = unsafe { hwnd.handle() } as HWND;
    unsafe {
        let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowBindData<T>;
        if ptr.is_null() {
            return The::from_raw(0 as _);
        }
        let dref = ptr.as_mut().unwrap();
        assert_eq!(
            dref.type_name,
            std::any::type_name::<T>(),
            "类型断言失败: 过程回调函数与窗口构建器各自指定的窗口绑定类型类型不一致"
        );
        The::from_raw(dref.data_ptr as _)
    }
}
