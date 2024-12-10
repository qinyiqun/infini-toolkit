use crate::bindings::DataLayout;
use digit_layout::{types as ty, DigitLayout};
use std::{mem::transmute, sync::LazyLock};

#[allow(dead_code)]
pub(crate) fn digit_layout(dt: DataLayout) -> DigitLayout {
    let code: u32 = unsafe { transmute(dt) };
    macro_rules! match_dt {
        ($( $name:ident )+) => {
            $( if code == *$name { return ty::$name; } )+
        };
    }
    match_dt!(I8 I16 I32 I64 U8 U16 U32 U64 F16 BF16 F32 F64);
    panic!("unsupported data type")
}

pub(crate) fn data_layout(dt: DigitLayout) -> DataLayout {
    macro_rules! match_dt {
        ($( $name:ident )+) => {
            match dt {
                $( ty::$name => unsafe { transmute::<u32, DataLayout>(*$name) }, )+
                _ => panic!("unsupported data type"),
            }
        };
    }
    match_dt!(I8 I16 I32 I64 U8 U16 U32 U64 F16 BF16 F32 F64)
}

macro_rules! dt {
    ($( $name:ident = $packed:expr, $sign:expr, $size:expr, $mantissa:expr, $exponent:expr )+) => {
        $(
            static $name: LazyLock<u32> = LazyLock::new(move || {
                let dt = DataLayout {
                    _bitfield_align_1: [],
                    _bitfield_1: DataLayout::new_bitfield_1($packed, $sign, $size, $mantissa, $exponent),
                };
                unsafe { transmute(dt) }
            });
        )+
    };
}

dt! {
    I8   = 1, 1, 1,  7,  0
    I16  = 1, 1, 2, 15,  0
    I32  = 1, 1, 4, 31,  0
    I64  = 1, 1, 8, 63,  0
    U8   = 1, 0, 1,  8,  0
    U16  = 1, 0, 2, 16,  0
    U32  = 1, 0, 4, 32,  0
    U64  = 1, 0, 8, 64,  0
    F16  = 1, 1, 2, 10,  5
    BF16 = 1, 1, 2,  7,  8
    F32  = 1, 1, 4, 23,  8
    F64  = 1, 1, 8, 52, 11
}
