#![cfg(infini)]
#![deny(warnings)]

use infini_rt::{DevByte, Stream};
use std::{ffi::c_uint, ptr::null_mut};

#[macro_use]
#[allow(non_snake_case, non_camel_case_types)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[macro_export]
    macro_rules! infiniccl {
        ($f:expr) => {{
            #[allow(unused_imports)]
            use $crate::bindings::*;
            #[allow(unused_unsafe, clippy::macro_metavars_in_unsafe)]
            let err = unsafe { $f };
            assert_eq!(err, infinicclStatus_t::INFINICCL_STATUS_SUCCESS);
        }};
    }
}

#[repr(transparent)]
pub struct Comm(bindings::infinicclComm_t);

impl Comm {
    pub fn init_all(ty: bindings::DeviceType, indices: &[c_uint]) -> Vec<Self> {
        let mut ans = vec![null_mut(); indices.len()];
        infiniccl!(infinicclCommInitAll(
            ty,
            ans.as_mut_ptr(),
            indices.len() as _,
            indices.as_ptr()
        ));
        ans.into_iter().map(Self).collect()
    }
}

impl Drop for Comm {
    fn drop(&mut self) {
        infiniccl!(infinicclCommDestroy(self.0))
    }
}

unsafe impl Send for Comm {}
unsafe impl Sync for Comm {}

impl AsRaw for Comm {
    type Raw = bindings::infinicclComm_t;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl Comm {
    pub fn allreduce_sum(
        &self,
        recvbuf: &mut [DevByte],
        sendbuf: &[DevByte],
        dt: bindings::InfiniDataType_t,
        stream: &Stream,
    ) {
        use infini_rt::AsRaw;
        infiniccl!(infinicclAllReduceSum(
            self.as_raw(),
            sendbuf.as_ptr().cast_mut().cast(),
            recvbuf.as_mut_ptr().cast(),
            sendbuf.len(),
            dt,
            stream.as_raw().cast()
        ))
    }
}

/// 资源的原始形式的表示。通常来自底层库的定义。
pub trait AsRaw {
    /// 原始形式的类型。
    type Raw: Unpin + 'static;
    /// # Safety
    ///
    /// The caller must ensure that the returned item is dropped before the original item.
    unsafe fn as_raw(&self) -> Self::Raw;
}
