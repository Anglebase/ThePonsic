use winapi::{
    shared::windef::HWND,
    um::winuser::{GWLP_USERDATA, GetWindowLongPtrW, SetWindowLongPtrW},
};

use crate::WindowId;

pub(crate) struct WindowBindData<T> {
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

    pub(crate) fn free(&self) {
        unsafe {
            let _ = Box::from_raw(self.data_ptr as *mut T);
        }
    }
}

pub fn attach<T>(id: WindowId, data: T) {
    let hwnd = unsafe { id.handle() as HWND };
    let data_ptr = Box::into_raw(Box::new(WindowBindData::new(data)));
    unsafe {
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, data_ptr as _);
    }
}

pub unsafe fn cast_uncheck<T>(id: &WindowId) -> Option<&mut T> {
    let hwnd = unsafe { id.handle() } as HWND;
    let warpper_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut WindowBindData<T>;
    if warpper_ptr.is_null() {
        return None;
    }
    let warpper_ptr = unsafe { warpper_ptr.as_mut().unwrap() };
    let data_ptr = warpper_ptr.data_ptr;
    Some(unsafe { data_ptr.as_mut().unwrap() })
}

pub fn assert_cast<T>(id: &WindowId) -> Option<&mut T> {
    let hwnd = unsafe { id.handle() } as HWND;
    let warpper_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut WindowBindData<T>;
    if warpper_ptr.is_null() {
        return None;
    }
    let warpper_ptr = unsafe { warpper_ptr.as_mut().unwrap() };
    let data_ptr = warpper_ptr.data_ptr;
    assert_eq!(warpper_ptr.type_name, std::any::type_name::<T>());
    Some(unsafe { data_ptr.as_mut().unwrap() })
}

pub fn cast<T>(id: &WindowId) -> Option<&mut T> {
    let hwnd = unsafe { id.handle() } as HWND;
    let warpper_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut WindowBindData<T>;
    if warpper_ptr.is_null() {
        return None;
    }
    let warpper_ptr = unsafe { warpper_ptr.as_mut().unwrap() };
    let data_ptr = warpper_ptr.data_ptr;
    if warpper_ptr.type_name != std::any::type_name::<T>() {
        return None;
    }
    Some(unsafe { data_ptr.as_mut().unwrap() })
}

pub unsafe fn free_data<T>(id: &WindowId) {
    let hwnd = unsafe { id.handle() } as HWND;
    let warpper_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut WindowBindData<T>;
    if warpper_ptr.is_null() {
        return;
    }
    let warpper_ptr = unsafe { warpper_ptr.as_mut().unwrap() };
    warpper_ptr.free();
}