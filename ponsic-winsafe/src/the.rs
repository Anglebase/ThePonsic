use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

/// 内存访问器
///
/// 此结构体是类似于`Box<T>`的智能指针，但是该指针不持有所有权，不参与内存申请与释放的管理，仅用于访问既分配的内存
pub struct The<T> {
    the: Option<Box<T>>,
}

impl<T> Debug for The<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.the {
            Some(_) => f.debug_struct("The<T>").field("the", &"Some(...)").finish(),
            None => f.debug_struct("The<T>").field("the", &"None").finish(),
        }
    }
}

impl<T> Display for The<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.the {
            Some(_) => f.write_str("The<T> Some(...)"),
            None => f.write_str("The<T> None"),
        }
    }
}

impl<T> The<T> {
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self {
            the: if ptr.is_null() {
                None
            } else {
                Some(unsafe { Box::from_raw(ptr) })
            },
        }
    }

    /// 判断`The<T>`是否包含值
    pub fn has_value(&self) -> bool {
        self.the.is_some()
    }

    /// 获取`The<T>`的不可变引用访问器
    pub fn as_ref(&self) -> Option<TheRef<T>> {
        match self.the {
            Some(_) => Some(TheRef { the: self }),
            None => None,
        }
    }

    /// 获取`The<T>`的可变引用访问器
    pub fn as_mut(&mut self) -> Option<TheMut<T>> {
        match self.the {
            Some(_) => Some(TheMut { the: self }),
            None => None,
        }
    }
}

impl<T> Drop for The<T> {
    fn drop(&mut self) {
        if let Some(the) = self.the.take() {
            let _ = Box::into_raw(the);
        }
    }
}

/// 参考 `The<T>`
pub struct TheRef<'a, T> {
    the: &'a The<T>,
}

impl<'a, T> Deref for TheRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.the.the.as_ref().unwrap()
    }
}

/// 参考 `The<T>`
pub struct TheMut<'a, T> {
    the: &'a mut The<T>,
}

impl<'a, T> Deref for TheMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.the.the.as_ref().unwrap()
    }
}

impl<'a, T> DerefMut for TheMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.the.the.as_mut().unwrap()
    }
}
