pub mod primitives;

use std::fmt::Debug;

pub trait PlcDataType:
    Clone + Debug + Default + zerocopy::AsBytes + zerocopy::FromBytes + zerocopy::FromZeroes
{
    fn size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Self::read_from(bytes)
    }
}
