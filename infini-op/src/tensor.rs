use crate::{bindings::infiniopTensorDescriptor_t, data_layout::data_layout, AsRaw};
use digit_layout::DigitLayout;
use std::ptr::null_mut;

#[repr(transparent)]
pub struct Tensor(infiniopTensorDescriptor_t);

impl Tensor {
    pub fn new(
        dt: DigitLayout,
        shape: impl IntoIterator<Item = usize>,
        strides: impl IntoIterator<Item = isize>,
    ) -> Self {
        let ele = dt.nbytes() as isize;
        let shape: Vec<_> = shape.into_iter().map(|x| x as _).collect();
        let strides: Vec<_> = strides.into_iter().map(|x| (x / ele) as _).collect();
        let ndim = shape.len();
        assert_eq!(strides.len(), ndim);

        let mut ptr = null_mut();
        infiniop!(infiniopCreateTensorDescriptor(
            &mut ptr,
            ndim as _,
            shape.as_ptr(),
            strides.as_ptr(),
            data_layout(dt),
        ));
        Self(ptr)
    }
}

impl Drop for Tensor {
    fn drop(&mut self) {
        infiniop!(infiniopDestroyTensorDescriptor(self.0))
    }
}

unsafe impl Send for Tensor {}
unsafe impl Sync for Tensor {}

impl AsRaw for Tensor {
    type Raw = infiniopTensorDescriptor_t;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0
    }
}
