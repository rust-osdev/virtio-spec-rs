use volatile::access::{ReadOnly, ReadWrite};
use volatile_macro::VolatileFieldAccess;

pub use super::features::balloon::F;
use crate::le32;

/// Traditional Memory Balloon Device Configuration Layout
///
/// Use [`ConfigVolatileFieldAccess`] to work with this struct.
#[doc(alias = "virtio_balloon_config")]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy_derive::KnownLayout,
        zerocopy_derive::Immutable,
        zerocopy_derive::FromBytes,
    )
)]
#[derive(VolatileFieldAccess)]
#[repr(C)]
pub struct Config {
    #[access(ReadOnly)]
    num_pages: le32,

    #[access(ReadWrite)]
    actual: le32,

    #[access(ReadOnly)]
    free_page_hint_cmd_id: le32,

    #[access(ReadWrite)]
    poison_val: le32,
}
