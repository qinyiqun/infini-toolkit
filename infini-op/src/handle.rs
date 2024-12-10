use crate::{
    bindings::{infiniopHandle_t, Device},
    AsRaw,
};
use std::{ffi::c_int, ptr::null_mut};

#[repr(transparent)]
pub struct Handle(infiniopHandle_t);

impl Handle {
    pub fn new(device: Device, id: c_int) -> Self {
        let mut ptr = null_mut();
        infiniop!(infiniopCreateHandle(&mut ptr, device, id));
        Self(ptr)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        infiniop!(infiniopDestroyHandle(self.0))
    }
}

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl AsRaw for Handle {
    type Raw = infiniopHandle_t;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0
    }
}
