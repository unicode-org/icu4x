pub trait UnalignedLE where Self: Sized {
    type Error;

    fn parse_bytes(bytes: &[u8]) -> Result<&[Self], Self::Error>;
    fn as_bytes(slice: &[Self]) -> &[u8];
}

pub trait AsUnalignedLE {
    type UnalignedLE: UnalignedLE;

    fn into_unaligned(self) -> Self::UnalignedLE;
    fn from_unaligned(unaligned: Self::UnalignedLE) -> Self;
}
