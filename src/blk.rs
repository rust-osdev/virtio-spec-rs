//! Block Device

use bitfield_struct::bitfield;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use volatile::access::ReadOnly;
use volatile_macro::VolatileFieldAccess;

pub use super::features::blk::F;
use crate::{le16, le32, le64};

/// Block Device Configuration Layout
///
/// Use [`ConfigVolatileFieldAccess`] to work with this struct.
#[doc(alias = "virtio_blk_config")]
#[derive(VolatileFieldAccess)]
#[repr(C)]
pub struct Config {
    #[access(ReadOnly)]
    capacity: le64,

    #[access(ReadOnly)]
    size_max: le32,

    #[access(ReadOnly)]
    seg_max: le32,

    #[access(ReadOnly)]
    geometry: Geometry,

    #[access(ReadOnly)]
    blk_size: le32,

    #[access(ReadOnly)]
    topology: Topology,

    #[access(ReadOnly)]
    writeback: u8,

    #[access(ReadOnly)]
    unused0: u8,

    #[access(ReadOnly)]
    num_queues: u8,

    #[access(ReadOnly)]
    max_discard_sectors: le32,

    #[access(ReadOnly)]
    max_discard_seg: le32,

    #[access(ReadOnly)]
    discard_sector_alignment: le32,

    #[access(ReadOnly)]
    max_write_zeroes_sectors: le32,

    #[access(ReadOnly)]
    max_write_zeroes_seg: le32,

    #[access(ReadOnly)]
    write_zeroes_may_unmap: u8,

    #[access(ReadOnly)]
    unused1: [u8; 3],

    #[access(ReadOnly)]
    max_secure_erase_sectors: le32,

    #[access(ReadOnly)]
    max_secure_erase_seg: le32,

    #[access(ReadOnly)]
    secure_erase_sector_alignment: le32,

    #[access(ReadOnly)]
    zoned: ZonedCharacteristics,
}

/// Disk-style geometry.
#[doc(alias = "virtio_blk_geometry")]
#[derive(VolatileFieldAccess)]
#[repr(C)]
pub struct Geometry {
    #[access(ReadOnly)]
    cylinders: le16,

    #[access(ReadOnly)]
    heads: u8,

    #[access(ReadOnly)]
    sectors: u8,
}

/// Block device topology.
#[doc(alias = "virtio_blk_topology")]
#[derive(VolatileFieldAccess)]
#[repr(C)]
pub struct Topology {
    /// # of logical blocks per physical block (log2)
    #[access(ReadOnly)]
    physical_block_exp: u8,

    /// offset of first aligned logical block
    #[access(ReadOnly)]
    alignment_offset: u8,

    /// suggested minimum I/O size in blocks
    #[access(ReadOnly)]
    min_io_size: le16,

    /// optimal (suggested maximum) I/O size in blocks
    #[access(ReadOnly)]
    opt_io_size: le32,
}

#[doc(alias = "virtio_blk_zoned_characteristics")]
#[derive(VolatileFieldAccess)]
#[repr(C)]
pub struct ZonedCharacteristics {
    #[access(ReadOnly)]
    zone_sectors: le32,

    #[access(ReadOnly)]
    max_open_zones: le32,

    #[access(ReadOnly)]
    max_active_zones: le32,

    #[access(ReadOnly)]
    max_append_sectors: le32,

    #[access(ReadOnly)]
    write_granularity: le32,

    #[access(ReadOnly)]
    model: u8,

    #[access(ReadOnly)]
    unused2: [u8; 3],
}

/// A zoning model.
#[doc(alias = "VIRTIO_BLK_Z")]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum Z {
    #[doc(alias = "VIRTIO_BLK_Z_NONE")]
    None = 0,

    #[doc(alias = "VIRTIO_BLK_Z_HM")]
    Hm = 1,

    #[doc(alias = "VIRTIO_BLK_Z_HA")]
    Ha = 2,
}

/// The type of a request.
#[doc(alias = "VIRTIO_BLK_T")]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u32)]
pub enum T {
    #[doc(alias = "VIRTIO_BLK_T_IN")]
    In = 0,

    #[doc(alias = "VIRTIO_BLK_T_OUT")]
    Out = 1,

    #[doc(alias = "VIRTIO_BLK_T_FLUSH")]
    Flush = 4,

    #[doc(alias = "VIRTIO_BLK_T_GET_ID")]
    GetId = 8,

    #[doc(alias = "VIRTIO_BLK_T_GET_LIFETIME")]
    GetLifetime = 10,

    #[doc(alias = "VIRTIO_BLK_T_DISCARD")]
    Discard = 11,

    #[doc(alias = "VIRTIO_BLK_T_WRITE_ZEROES")]
    WriteZeroes = 13,

    #[doc(alias = "VIRTIO_BLK_T_SECURE_ERASE")]
    SecureErase = 14,

    #[doc(alias = "VIRTIO_BLK_T_ZONE_APPEND")]
    ZoneAppend = 15,

    #[doc(alias = "VIRTIO_BLK_T_ZONE_REPORT")]
    ZoneReport = 16,

    #[doc(alias = "VIRTIO_BLK_T_ZONE_OPEN")]
    ZoneOpen = 18,

    #[doc(alias = "VIRTIO_BLK_T_ZONE_CLOSE")]
    ZoneClose = 20,

    #[doc(alias = "VIRTIO_BLK_T_ZONE_FINISH")]
    ZoneFinish = 22,

    #[doc(alias = "VIRTIO_BLK_T_ZONE_RESET")]
    ZoneReset = 24,

    #[doc(alias = "VIRTIO_BLK_T_ZONE_RESET_ALL")]
    ZoneResetAll = 26,
}

/// A segment for discard, secure erase or write zeroes.
#[doc(alias = "virtio_blk_discard_write_zeroes")]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy_derive::KnownLayout,
        zerocopy_derive::Immutable,
        zerocopy_derive::FromBytes,
        zerocopy_derive::IntoBytes,
    )
)]
#[repr(C)]
pub struct DiscardWriteZeroes {
    pub sector: le64,
    pub num_sectors: le32,
    pub flags: le32,
}

/// The flags of a [`DiscardWriteZeroes`] segment.
#[bitfield(u32, repr = le32, from = le32::from_ne, into = le32::to_ne)]
pub struct DiscardWriteZeroesFlags {
    #[bits(1)]
    pub unmap: bool,

    #[bits(31)]
    pub reserved: u32,
}

/// Lifetime data populated by the device.
#[doc(alias = "virtio_blk_lifetime")]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy_derive::KnownLayout,
        zerocopy_derive::Immutable,
        zerocopy_derive::FromBytes,
        zerocopy_derive::IntoBytes,
    )
)]
#[repr(C)]
pub struct Lifetime {
    pub pre_eol_info: le16,
    pub device_lifetime_est_typ_a: le16,
    pub device_lifetime_est_typ_b: le16,
}

/// End-of-life information.
#[doc(alias = "VIRTIO_BLK_PRE_EOL_INFO")]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u16)]
pub enum PreEolInfo {
    /// Value not available
    #[doc(alias = "VIRTIO_BLK_PRE_EOL_INFO_UNDEFINED")]
    Undefined = 0,

    /// < 80% of reserved blocks are consumed
    #[doc(alias = "VIRTIO_BLK_PRE_EOL_INFO_NORMAL")]
    Normal = 1,

    /// 80% of reserved blocks are consumed
    #[doc(alias = "VIRTIO_BLK_PRE_EOL_INFO_WARNING")]
    Warning = 2,

    /// 90% of reserved blocks are consumed
    #[doc(alias = "VIRTIO_BLK_PRE_EOL_INFO_URGENT")]
    Urgent = 3,
}

/// A status byte.
#[doc(alias = "VIRTIO_BLK_S")]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum S {
    #[doc(alias = "VIRTIO_BLK_S_OK")]
    Ok = 0,

    #[doc(alias = "VIRTIO_BLK_S_IOERR")]
    Ioerr = 1,

    #[doc(alias = "VIRTIO_BLK_S_UNSUPP")]
    Unsupp = 2,

    #[doc(alias = "VIRTIO_BLK_S_ZONE_INVALID_CMD")]
    ZoneInvalidCmd = 3,

    #[doc(alias = "VIRTIO_BLK_S_ZONE_UNALIGNED_WP")]
    ZoneUnalignedWp = 4,

    #[doc(alias = "VIRTIO_BLK_S_ZONE_OPEN_RESOURCE")]
    ZoneOpenResource = 5,

    #[doc(alias = "VIRTIO_BLK_S_ZONE_ACTIVE_RESOURCE")]
    ZoneActiveResource = 6,
}

/// A zone descriptor.
#[doc(alias = "virtio_blk_zone_descriptor")]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy_derive::KnownLayout,
        zerocopy_derive::Immutable,
        zerocopy_derive::FromBytes,
        zerocopy_derive::IntoBytes,
    )
)]
#[repr(C)]
pub struct ZoneDescriptor {
    z_cap: le64,
    z_start: le64,
    z_wp: le64,
    z_type: u8,
    z_state: u8,
    reserved: [u8; 38],
}

/// A zone type.
#[doc(alias = "VIRTIO_BLK_ZT")]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum Zt {
    #[doc(alias = "VIRTIO_BLK_ZT_CONV")]
    Conv = 1,

    #[doc(alias = "VIRTIO_BLK_ZT_SWR")]
    Swr = 2,

    #[doc(alias = "VIRTIO_BLK_ZT_SWP")]
    Swp = 3,
}

/// A zone state.
#[doc(alias = "VIRTIO_BLK_ZS")]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum Zs {
    #[doc(alias = "VIRTIO_BLK_ZS_NOT_WP")]
    NotWp = 0,

    #[doc(alias = "VIRTIO_BLK_ZS_EMPTY")]
    Empty = 1,

    #[doc(alias = "VIRTIO_BLK_ZS_IOPEN")]
    Iopen = 2,

    #[doc(alias = "VIRTIO_BLK_ZS_EOPEN")]
    Eopen = 3,

    #[doc(alias = "VIRTIO_BLK_ZS_CLOSED")]
    Closed = 4,

    #[doc(alias = "VIRTIO_BLK_ZS_RDONLY")]
    Rdonly = 13,

    #[doc(alias = "VIRTIO_BLK_ZS_FULL")]
    Full = 14,

    #[doc(alias = "VIRTIO_BLK_ZS_OFFLINE")]
    Offline = 15,
}
