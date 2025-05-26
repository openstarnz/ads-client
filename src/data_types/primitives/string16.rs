/**
 * A latin-1 16 Character String for Beckhoff ADS. Includes a 0 for null termination.
 */
use anyhow::{anyhow, bail};
use zerocopy::FromZeroes;

use crate::data_types::PlcDataType;

#[derive(Clone, Debug, Default, zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
#[repr(C)]
pub struct PlcString16 {
    inner: [u8; 16],
    null_terminator: u8, // The PLC spec includes one byte of null termination
}

impl PlcDataType for PlcString16 {}

impl TryFrom<String> for PlcString16 {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let src = value.as_bytes();
        let src_len = src.len();
        if src_len > 16 {
            return Err(anyhow!(
                "Could not convert from String. Longer than 16 bytes."
            ));
        }
        let mut blank = PlcString16::new_zeroed();
        // Does not use from_bytes as it does not need the length to match perfectly
        blank.inner[..src_len].copy_from_slice(src);
        Ok(blank)
    }
}

impl From<PlcString16> for String {
    fn from(val: PlcString16) -> Self {
        val.inner.into_iter().map(|c| c as char).collect()
    }
}

impl PlcString16 {
    // Returns an error if the null terminator is not zero.
    pub fn check_terminator(&self) -> anyhow::Result<()> {
        if self.null_terminator != 0 {
            bail!("Null terminator is {} not 0.", self.null_terminator);
        }
        Ok(())
    }
}
