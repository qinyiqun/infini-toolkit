use crate::{bindings::infiniopStatus_t, AsRaw};
use std::ptr::null_mut;

pub struct Descriptor<T> {
    ptr: *mut T,
    destroyer: unsafe extern "C" fn(*mut T) -> infiniopStatus_t,
}

impl<T> Descriptor<T> {
    pub fn new(
        f: impl FnOnce(&mut *mut T),
        destroyer: unsafe extern "C" fn(*mut T) -> infiniopStatus_t,
    ) -> Self {
        let mut ptr = null_mut();
        f(&mut ptr);
        Self { ptr, destroyer }
    }
}

impl<T> Drop for Descriptor<T> {
    fn drop(&mut self) {
        assert_eq!(
            unsafe { (self.destroyer)(self.ptr) },
            infiniopStatus_t::STATUS_SUCCESS
        )
    }
}

unsafe impl<T> Send for Descriptor<T> {}
unsafe impl<T> Sync for Descriptor<T> {}

impl<T: 'static> AsRaw for Descriptor<T> {
    type Raw = *mut T;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.ptr
    }
}
