use crate::{bindings::infinirtStream_t, AsRaw, Device};
use std::{ffi::c_void, ptr::null_mut};

#[repr(transparent)]
pub struct Stream(infinirtStream_t);

impl Device {
    pub fn stream(&self) -> Stream {
        let mut stream = null_mut();
        infini!(infinirtStreamCreate(&mut stream, self.ty, self.id));
        Stream(stream)
    }
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

impl Drop for Stream {
    fn drop(&mut self) {
        infini!(infinirtStreamDestroy(self.0))
    }
}

impl AsRaw for Stream {
    type Raw = infinirtStream_t;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl Stream {
    #[inline]
    pub fn synchronize(&self) {
        infini!(infinirtStreamSynchronize(self.0))
    }

    #[inline]
    pub fn get_device(&self) -> Device {
        let mut ty = crate::DeviceType::DEVICE_CPU;
        let mut id = 0;
        infini!(infinirtGetStreamDeviceInfo(&mut ty, &mut id, self.0));
        Device { ty, id }
    }

    /// 获取相对 infinirt 更底层的流对象指针。
    ///
    /// # Safety
    ///
    /// 调用者必须保证指针的正确使用，否则可能导致未定义行为。
    #[inline]
    pub unsafe fn as_void_ptr(&self) -> *mut c_void {
        let mut ptr = null_mut();
        infini!(infinirtGetRawStream(&mut ptr, self.0));
        ptr
    }
}
