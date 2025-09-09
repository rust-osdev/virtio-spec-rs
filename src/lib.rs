//! This crate contains the Rust equivalents of the definitions from the [Virtual I/O Device (VIRTIO) Specification](https://github.com/oasis-tcs/virtio-spec).
//! This crate aims to be unopinionated regarding actual VIRTIO drivers that are implemented on top of this crate.
//!
//! # Usage
//!
//! We recommend to rename this crate to `virtio` when adding the dependency.
//! This allows closely matching the specification when using definitions:
//!
//! - `VIRTIO_NET_F_CSUM` from the specification becomes [`virtio::net::F::CSUM`] in this crate.
//! - `virtio_net_config` from the specification becomes [`virtio::net::Config`] in this crate.
//!  
//! [`virtio::net::F::CSUM`]: net::F::CSUM
//! [`virtio::net::Config`]: net::Config
//!
//! Either run
//!
//! ```bash
//! cargo add virtio-spec --rename virtio
//! ```
//!
//! or manually edit your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! virtio = { package = "virtio-spec", version = "x.y.z" }
//! ```
//!
//! ## Features
//!
//! This crate has the following Cargo features:
//!
//! - `alloc` enables allocating unsized structs such as [`virtq::Avail`] and [`virtq::Used`] via the [`allocator_api2`] crate.
//! - `mmio` enables the [`mmio`] module for Virtio Over MMIO.
//! - `nightly` enables nightly-only functionality.
//! - `pci` enables the [`pci`] module for Virtio Over PCI via the [`pci_types`] crate.
//! - `zerocopy` derives the following traits for most structs:
//!   - [`zerocopy::KnownLayout`]
//!   - [`zerocopy::Immutable`]
//!   - [`zerocopy::FromBytes`]
//!   - [`zerocopy::IntoBytes`]
//!
//! # Implementation Status
//!
//! This crate adds new modules by demand.
//! If you need anything that is not available yet, please open an issue.
//!
//! ## Virtqueues
//!
//! | Virtqueue         | Available | Module     |
//! | ----------------- | --------- | ---------- |
//! | Split Virtqueues  | ✅        | [`virtq`]  |
//! | Packed Virtqueues | ✅        | [`pvirtq`] |
//!
//! ## Transport Options
//!
//! | Transport Option | Available | Module   |
//! | ---------------- | --------- | -------- |
//! | PCI Bus          | ✅        | [`pci`]  |
//! | MMIO             | ✅        | [`mmio`] |
//! | Channel I/O      | ❌        |          |
//!
//! ## Device Types
//!
//! | Device Type                       | Available | Module      |
//! | --------------------------------- | --------- | ----------- |
//! | Network Device                    | ✅        | [`net`]     |
//! | Block Device                      | ❌        |             |
//! | Console Device                    | ✅        | [`console`] |
//! | Entropy Device                    | ❌        |             |
//! | Traditional Memory Balloon Device | ✅        | [`balloon`] |
//! | SCSI Host Device                  | ❌        |             |
//! | GPU Device                        | ❌        |             |
//! | Input Device                      | ❌        |             |
//! | Crypto Device                     | ❌        |             |
//! | Socket Device                     | ✅        | [`vsock`]   |
//! | File System Device                | ✅        | [`fs`]      |
//! | RPMB Device                       | ❌        |             |
//! | IOMMU Device                      | ❌        |             |
//! | Sound Device                      | ❌        |             |
//! | Memory Device                     | ❌        |             |
//! | I2C Adapter Device                | ❌        |             |
//! | SCMI Device                       | ❌        |             |
//! | GPIO Device                       | ❌        |             |
//! | PMEM Device                       | ❌        |             |

#![cfg_attr(not(test), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod bitflags;
#[macro_use]
pub mod volatile;
pub mod balloon;
pub mod console;
#[cfg(any(feature = "mmio", feature = "pci"))]
mod driver_notifications;
mod features;
pub mod fs;
#[cfg(feature = "mmio")]
pub mod mmio;
pub mod net;
#[cfg(feature = "pci")]
pub mod pci;
pub mod pvirtq;
pub mod virtq;
pub mod vsock;

pub use endian_num::{be128, be16, be32, be64, le128, le16, le32, le64, Be, Le};
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

pub use self::features::{FeatureBits, F};

virtio_bitflags! {
    /// Device Status Field
    ///
    /// During device initialization by a driver,
    /// the driver follows the sequence of steps specified in
    /// _General Initialization And Device Operation / Device
    /// Initialization_.
    ///
    /// The `device status` field provides a simple low-level
    /// indication of the completed steps of this sequence.
    /// It's most useful to imagine it hooked up to traffic
    /// lights on the console indicating the status of each device.  The
    /// following bits are defined (listed below in the order in which
    /// they would be typically set):
    pub struct DeviceStatus: u8 {
        /// Indicates that the guest OS has found the
        /// device and recognized it as a valid virtio device.
        const ACKNOWLEDGE = 1;

        /// Indicates that the guest OS knows how to drive the
        /// device.
        ///
        /// <div class="warning">
        ///
        /// There could be a significant (or infinite) delay before setting
        /// this bit.  For example, under Linux, drivers can be loadable modules.
        ///
        /// </div>
        const DRIVER = 2;

        /// Indicates that something went wrong in the guest,
        /// and it has given up on the device. This could be an internal
        /// error, or the driver didn't like the device for some reason, or
        /// even a fatal error during device operation.
        const FAILED = 128;

        /// Indicates that the driver has acknowledged all the
        /// features it understands, and feature negotiation is complete.
        const FEATURES_OK = 8;

        /// Indicates that the driver is set up and ready to
        /// drive the device.
        const DRIVER_OK = 4;

        /// Indicates that the device has experienced
        /// an error from which it can't recover.
        const DEVICE_NEEDS_RESET = 64;
    }
}

/// Virtio Device IDs
///
/// <div class="warning">
///
/// This enum is not ABI-compatible with it's corresponding field.
/// Use [`Id::from`] for converting from an integer.
///
/// </div>
///
/// [`Id::from`]: Id#impl-From<u8>-for-Id
#[derive(IntoPrimitive, FromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum Id {
    /// reserved (invalid)
    Reserved = 0,

    /// network card
    Net = 1,

    /// block device
    Block = 2,

    /// console
    Console = 3,

    /// entropy source
    Rng = 4,

    /// memory ballooning (traditional)
    Balloon = 5,

    /// ioMemory
    IoMem = 6,

    /// rpmsg
    Rpmsg = 7,

    /// SCSI host
    Scsi = 8,

    /// 9P transport
    NineP = 9,

    /// mac80211 wlan
    Mac80211Wlan = 10,

    /// rproc serial
    RprocSerial = 11,

    /// virtio CAIF
    Caif = 12,

    /// memory balloon
    MemoryBalloon = 13,

    /// GPU device
    Gpu = 16,

    /// Timer/Clock device
    Clock = 17,

    /// Input device
    Input = 18,

    /// Socket device
    Vsock = 19,

    /// Crypto device
    Crypto = 20,

    /// Signal Distribution Module
    SignalDist = 21,

    /// pstore device
    Pstore = 22,

    /// IOMMU device
    Iommu = 23,

    /// Memory device
    Mem = 24,

    /// Audio device
    Sound = 25,

    /// file system device
    Fs = 26,

    /// PMEM device
    Pmem = 27,

    /// RPMB device
    Rpmb = 28,

    /// mac80211 hwsim wireless simulation device
    Mac80211Hwsim = 29,

    /// Video encoder device
    VideoEncoder = 30,

    /// Video decoder device
    VideoDecoder = 31,

    /// SCMI device
    Scmi = 32,

    /// NitroSecureModule
    NitroSecMod = 33,

    /// I2C adapter
    I2cAdapter = 34,

    /// Watchdog
    Watchdog = 35,

    /// CAN device
    Can = 36,

    /// Parameter Server
    ParamServ = 38,

    /// Audio policy device
    AudioPolicy = 39,

    /// Bluetooth device
    Bt = 40,

    /// GPIO device
    Gpio = 41,

    /// RDMA device
    Rdma = 42,

    /// Unknown device
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Descriptor Ring Change Event Flags
#[doc(alias = "RING_EVENT_FLAGS")]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum RingEventFlags {
    /// Enable events
    #[doc(alias = "RING_EVENT_FLAGS_ENABLE")]
    Enable = 0x0,

    /// Disable events
    #[doc(alias = "RING_EVENT_FLAGS_DISABLE")]
    Disable = 0x1,

    /// Enable events for a specific descriptor
    /// (as specified by Descriptor Ring Change Event Offset/Wrap Counter).
    /// Only valid if VIRTIO_F_EVENT_IDX has been negotiated.
    #[doc(alias = "RING_EVENT_FLAGS_DESC")]
    Desc = 0x2,

    Reserved = 0x3,
}

impl RingEventFlags {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0x0 => Self::Enable,
            0x1 => Self::Disable,
            0x2 => Self::Desc,
            0x3 => Self::Reserved,
            _ => unreachable!(),
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Common device configuration space functionality.
pub trait DeviceConfigSpace: Sized {
    /// Read from device configuration space.
    ///
    /// This function should be used when reading from fields greater than
    /// 32 bits wide or when reading from multiple fields.
    ///
    /// As described in _Driver Requirements: Device Configuration Space_,
    /// this method checks the configuration atomicity value of the device
    /// and only returns once the value was the same before and after the
    /// provided function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use virtio_spec as virtio;
    /// use virtio::net::ConfigVolatileFieldAccess;
    /// use virtio::DeviceConfigSpace;
    /// use volatile::access::ReadOnly;
    /// use volatile::VolatilePtr;
    ///
    /// fn read_mac(
    ///     common_cfg: VolatilePtr<'_, virtio::pci::CommonCfg, ReadOnly>,
    ///     net_cfg: VolatilePtr<'_, virtio::net::Config, ReadOnly>,
    /// ) -> [u8; 6] {
    ///     common_cfg.read_config_with(|| net_cfg.mac().read())
    /// }
    /// ```
    fn read_config_with<F, T>(self, f: F) -> T
    where
        F: FnMut() -> T;
}
