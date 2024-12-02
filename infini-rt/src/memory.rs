use crate::{AsRaw, Device};
use std::{
    alloc::Layout,
    ops::{Deref, DerefMut},
    ptr::{null_mut, NonNull},
    slice::{from_raw_parts, from_raw_parts_mut},
};

#[repr(transparent)]
pub struct DevByte(u8);

pub struct DevBlob {
    dev: Device,
    ptr: NonNull<DevByte>,
    len: usize,
}

unsafe impl Send for DevBlob {}
unsafe impl Sync for DevBlob {}

impl Device {
    pub fn malloc<T: Copy>(&self, len: usize) -> DevBlob {
        let layout = Layout::array::<T>(len).unwrap();
        let len = layout.size();

        let mut ptr = null_mut();
        infini!(infinirtMalloc(&mut ptr, self.ty, self.id, len));

        DevBlob {
            dev: *self,
            ptr: NonNull::new(ptr).unwrap().cast(),
            len,
        }
    }

    pub fn from_host<T: Copy>(&self, data: &[T]) -> DevBlob {
        let len = size_of_val(data);
        let mut ptr = null_mut();
        infini!(infinirtMalloc(&mut ptr, self.ty, self.id, len));
        infini!(infinirtMemcpyH2D(
            ptr,
            self.ty,
            self.id,
            data.as_ptr().cast(),
            len
        ));
        DevBlob {
            dev: *self,
            ptr: NonNull::new(ptr).unwrap().cast(),
            len,
        }
    }

    pub fn memcpyh2d<T: Copy>(&self, dst: &mut [DevByte], src: &[T]) {
        let len = dst.len();
        assert_eq!(len, size_of_val(src));
        infini!(infinirtMemcpyH2D(
            dst.as_mut_ptr().cast(),
            self.ty,
            self.id,
            src.as_ptr().cast(),
            len,
        ))
    }

    pub fn memcpyd2h<T: Copy>(&self, dst: &mut [T], src: &[DevByte]) {
        let len = src.len();
        assert_eq!(len, size_of_val(dst));
        infini!(infinirtMemcpyD2H(
            dst.as_mut_ptr().cast(),
            src.as_ptr().cast(),
            self.ty,
            self.id,
            len
        ))
    }
}

impl Drop for DevBlob {
    fn drop(&mut self) {
        infini!(infinirtFree(
            self.ptr.as_ptr().cast(),
            self.dev.ty,
            self.dev.id
        ))
    }
}

impl AsRaw for DevBlob {
    type Raw = *mut DevByte;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.ptr.as_ptr()
    }
}

impl Deref for DevBlob {
    type Target = [DevByte];
    #[inline]
    fn deref(&self) -> &Self::Target {
        let len = self.len;
        if len == 0 {
            &[]
        } else {
            unsafe { from_raw_parts(self.ptr.as_ptr(), len) }
        }
    }
}

impl DerefMut for DevBlob {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let len = self.len;
        if len == 0 {
            &mut []
        } else {
            unsafe { from_raw_parts_mut(self.ptr.as_ptr(), len) }
        }
    }
}
