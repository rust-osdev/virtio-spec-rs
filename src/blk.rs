//! Block Device

use num_enum::{IntoPrimitive, TryFromPrimitive};
use volatile::access::ReadOnly;
use volatile_macro::VolatileFieldAccess;

pub use super::features::blk::F;
use crate::{le16, le32, le64};

// Device configuration.

/// Modelled up to `num_queues`; the trailing discard and write-zeroes
/// parameters follow in the device's configuration space and are never
/// accessed. The fields in between are modelled even where the driver
/// ignores them, because a volatile field's offset is its position in this
/// struct — skipping one would misplace everything after it.
///
/// Use `ConfigVolatileFieldAccess` to work with this struct.
#[doc(alias = "virtio_blk_config")]
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
    /// The capacity of the device, expressed in 512-byte sectors.
    #[access(ReadOnly)]
    capacity: le64,

    /// The maximum size of any single segment. Only valid if
    /// `SIZE_MAX` was negotiated.
    #[access(ReadOnly)]
    size_max: le32,

    /// The maximum number of segments in a request. Only valid if
    /// `SEG_MAX` was negotiated.
    #[access(ReadOnly)]
    seg_max: le32,

    /// Cylinders of the device's geometry.
    #[access(ReadOnly)]
    cylinders: le16,

    /// Heads of the device's geometry.
    #[access(ReadOnly)]
    heads: u8,

    /// Sectors of the device's geometry.
    #[access(ReadOnly)]
    sectors: u8,

    /// The optimal I/O size in bytes. Only valid if `BLK_SIZE` was
    /// negotiated. This is *not* the unit the device addresses, which is
    /// always `SECTOR_SIZE`.
    #[access(ReadOnly)]
    blk_size: le32,

    /// Number of logical blocks per physical block, as a power of two.
    /// Only valid if `TOPOLOGY` was negotiated.
    #[access(ReadOnly)]
    physical_block_exp: u8,

    /// Offset of the first aligned logical block. Only valid if `TOPOLOGY`
    /// was negotiated.
    #[access(ReadOnly)]
    alignment_offset: u8,

    /// Suggested minimum I/O size in blocks. Only valid if `TOPOLOGY` was
    /// negotiated.
    #[access(ReadOnly)]
    min_io_size: le16,

    /// Suggested optimal I/O size in blocks. Only valid if `TOPOLOGY` was
    /// negotiated.
    #[access(ReadOnly)]
    opt_io_size: le32,

    /// Whether the device's cache writes back. Only valid if `CONFIG_WCE`
    /// was negotiated.
    #[access(ReadOnly)]
    writeback: u8,

    #[access(ReadOnly)]
    unused0: u8,

    /// The number of request virtqueues. Only valid if `MQ` was negotiated;
    /// the device exposes a single request queue otherwise.
    #[access(ReadOnly)]
    num_queues: le16,
}

/// Request types, the `type` field of the request header.
#[doc(alias = "VIRTIO_BLK_T")]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u32)]
pub enum RequestType {
    #[doc(alias = "VIRTIO_BLK_T_IN")]
    IN = 0,

    #[doc(alias = "VIRTIO_BLK_T_OUT")]
    OUT = 1,

    #[doc(alias = "VIRTIO_BLK_T_FLUSH")]
    FLUSH = 4,
}

/// The status byte the device writes as the last part of every request.
#[doc(alias = "VIRTIO_BLK_S")]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum Status {
    #[doc(alias = "VIRTIO_BLK_S_OK")]
    OK = 0,

    #[doc(alias = "VIRTIO_BLK_S_IOERR")]
    IOERR = 1,

    #[doc(alias = "VIRTIO_BLK_S_UNSUPP")]
    UNSUPP = 2,
}

/// The fixed-size header that starts every request.
#[doc(alias = "virtio_blk_req")]
#[derive(Debug)]
#[repr(C)]
pub struct RequestHeader {
    type_: le32,
    reserved: le32,
    sector: le64,
}

impl RequestHeader {
    pub fn new(type_: RequestType, sector: u64) -> Self {
        Self {
            type_: le32::from_ne(type_.into()),
            reserved: le32::from_ne(0),
            sector: le64::from_ne(sector),
        }
    }
}
