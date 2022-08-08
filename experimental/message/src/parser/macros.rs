// macro_rules! get_byte {
//     ($s:expr, $idx:expr) => {
//         $s.source.as_ref().as_bytes().get($idx)
//     };
// }

macro_rules! get_current_byte {
    ($s:expr) => {
        $s.source.byte_at($s.ptr)
    };
}
