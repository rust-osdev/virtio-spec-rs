use volatile_macro::VolatileFieldAccess;

pub use super::features::entropy::F;

/// Entropy Device Configuration Layout
///
/// Use [`ConfigVolatileFieldAccess`] to work with this struct.
#[doc(alias = "virtio_entropy_config")]
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
pub struct Config {}
