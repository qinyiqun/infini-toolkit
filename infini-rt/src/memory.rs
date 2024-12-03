use crate::{AsRaw, Device, Stream};
use std::{
    alloc::Layout,
    mem::forget,
    ops::{Deref, DerefMut},
    os::raw::c_void,
    ptr::{null_mut, NonNull},
    slice::{from_raw_parts, from_raw_parts_mut},
};

#[repr(transparent)]
pub struct DevByte(u8);

impl Device {
    #[inline]
    pub fn memcpy(&self, dst: &mut [DevByte], src: &[DevByte]) {
        let (dst, src, len) = memcpy_ptr(dst, src);
        infini!(infinirtMemcpy(dst, src, self.ty, self.id, len,))
    }

    #[inline]
    pub fn memcpyh2d<T: Copy>(&self, dst: &mut [DevByte], src: &[T]) {
        let (dst, src, len) = memcpy_ptr(dst, src);
        infini!(infinirtMemcpyH2D(dst, self.ty, self.id, src, len,))
    }

    #[inline]
    pub fn memcpyd2h<T: Copy>(&self, dst: &mut [T], src: &[DevByte]) {
        let (dst, src, len) = memcpy_ptr(dst, src);
        infini!(infinirtMemcpyD2H(dst, src, self.ty, self.id, len))
    }
}

impl Stream {
    #[inline]
    pub fn memcpy(&self, dst: &mut [DevByte], src: &[DevByte]) {
        let (dst, src, len) = memcpy_ptr(dst, src);
        let Device { ty, id } = self.get_device();
        infini!(infinirtMemcpyAsync(dst, src, ty, id, len, self.as_raw()))
    }

    #[inline]
    pub fn memcpyh2d<T: Copy>(&self, dst: &mut [DevByte], src: &[T]) {
        let (dst, src, len) = memcpy_ptr(dst, src);
        let Device { ty, id } = self.get_device();
        infini!(infinirtMemcpyH2DAsync(dst, ty, id, src, len, self.as_raw()))
    }
}

#[inline]
fn memcpy_ptr<T, U>(dst: &mut [T], src: &[U]) -> (*mut c_void, *const c_void, usize) {
    let len = size_of_val(dst);
    assert_eq!(len, size_of_val(src));
    (dst.as_mut_ptr().cast(), src.as_ptr().cast(), len)
}

pub struct DevBlob {
    dev: Device,
    ptr: NonNull<DevByte>,
    len: usize,
}

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
        let src = data.as_ptr().cast();
        let len = size_of_val(data);

        let mut ptr = null_mut();
        infini!(infinirtMalloc(&mut ptr, self.ty, self.id, len));
        infini!(infinirtMemcpyH2D(ptr, self.ty, self.id, src, len));

        DevBlob {
            dev: *self,
            ptr: NonNull::new(ptr).unwrap().cast(),
            len,
        }
    }
}

impl Stream {
    pub fn malloc<T: Copy>(&self, len: usize) -> DevBlob {
        let layout = Layout::array::<T>(len).unwrap();
        let len = layout.size();

        let dev = self.get_device();
        let raw = unsafe { self.as_raw() };
        let mut ptr = null_mut();
        infini!(infinirtMallocAsync(&mut ptr, dev.ty, dev.id, len, raw));

        let ptr = NonNull::new(ptr).unwrap().cast();
        DevBlob { dev, ptr, len }
    }

    pub fn from_host<T: Copy>(&self, data: &[T]) -> DevBlob {
        let src = data.as_ptr().cast();
        let len = size_of_val(data);

        let dev = self.get_device();
        let raw = unsafe { self.as_raw() };
        let mut ptr = null_mut();
        infini!(infinirtMallocAsync(&mut ptr, dev.ty, dev.id, len, raw));
        infini!(infinirtMemcpyH2DAsync(ptr, dev.ty, dev.id, src, len, raw));

        let ptr = NonNull::new(ptr).unwrap().cast();
        DevBlob { dev, ptr, len }
    }

    pub fn free(&self, blob: DevBlob) {
        let &DevBlob { dev, ptr, .. } = &blob;
        forget(blob);

        infini!(infinirtFreeAsync(
            ptr.as_ptr().cast(),
            dev.ty,
            dev.id,
            self.as_raw()
        ));
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

unsafe impl Send for DevBlob {}
unsafe impl Sync for DevBlob {}

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

pub struct HostBlob {
    dev: Device,
    ptr: NonNull<u8>,
    len: usize,
}

impl Device {
    pub fn malloc_host<T: Copy>(&self, len: usize) -> HostBlob {
        let layout = Layout::array::<T>(len).unwrap();
        let len = layout.size();

        let mut ptr = null_mut();
        infini!(infinirtMallocHost(&mut ptr, self.ty, self.id, len));

        HostBlob {
            dev: *self,
            ptr: NonNull::new(ptr).unwrap().cast(),
            len,
        }
    }
}

impl Drop for HostBlob {
    fn drop(&mut self) {
        infini!(infinirtFreeHost(
            self.ptr.as_ptr().cast(),
            self.dev.ty,
            self.dev.id,
        ))
    }
}

unsafe impl Send for HostBlob {}
unsafe impl Sync for HostBlob {}

impl AsRaw for HostBlob {
    type Raw = *mut u8;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.ptr.as_ptr()
    }
}

impl Deref for HostBlob {
    type Target = [u8];
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

impl DerefMut for HostBlob {
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
