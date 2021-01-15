pub use tinystr::TinyStr4;
use tinystr::TinyStr8;
use tinystr::TinyStr16;
use crate::{Optional, Slice};

#[no_mangle]
pub extern "C" fn construct_tinystr4(slice: Slice<u8>) -> Optional<TinyStr4> {
    TinyStr4::from_bytes(slice.to_slice()).ok().into()
}

#[no_mangle]
pub extern "C" fn construct_tinystr8(slice: Slice<u8>) -> Optional<TinyStr8> {
    TinyStr8::from_bytes(slice.to_slice()).ok().into()
}


#[no_mangle]
pub extern "C" fn construct_tinystr16(slice: Slice<u8>) -> Optional<TinyStr16> {
    TinyStr16::from_bytes(slice.to_slice()).ok().into()
}
