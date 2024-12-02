#![cfg(infini)]

#[macro_use]
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::approx_constant
)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[macro_export]
    macro_rules! infini {
        ($f:expr) => {{
            #[allow(unused_imports)]
            use $crate::bindings::*;
            #[allow(unused_unsafe, clippy::macro_metavars_in_unsafe)]
            let err = unsafe { $f };
            assert_eq!(err, infinirtStatus_t::INFINIRT_STATUS_SUCCESS);
        }};
    }
}

pub use bindings::DeviceType::{self, *};

#[inline]
pub fn init(dev: DeviceType) {
    infini!(infinirtInit(dev));
}

#[test]
fn test_init() {
    init(DEVICE_CPU)
}
