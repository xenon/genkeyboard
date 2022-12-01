use std::fmt;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use variant_count::VariantCount;

pub mod greek;
pub mod latin;
pub mod russian;

#[derive(
    clap::ArgEnum,
    Clone,
    Debug,
    enum_utils::FromStr,
    Eq,
    IntoPrimitive,
    PartialEq,
    TryFromPrimitive,
    VariantCount,
)]
#[repr(u8)]
pub enum Layout {
    Greek,
    Latin,
    Russian,
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
