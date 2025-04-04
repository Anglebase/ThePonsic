use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

/// `The<T>`是与`Box<T>`类似的智能指针，但它不会自动参与内存的申请与释放，仅仅是内存数据的临时持有者
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

    /// 由`The<T>`释放内存
    pub unsafe fn free(mut self) {
        if let Some(the) = self.the.take() {
            let _ = the;
        }
    }

    /// 判断`The<T>`是否包含值
    pub fn has_value(&self) -> bool {
        self.the.is_some()
    }

    pub fn as_ref(&self) -> Option<TheRef<T>> {
        match self.the {
            Some(_) => Some(TheRef { the: self }),
            None => None,
        }
    }

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

pub struct TheRef<'a, T> {
    the: &'a The<T>,
}

impl<'a, T> Deref for TheRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.the.the.as_ref().unwrap()
    }
}

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
