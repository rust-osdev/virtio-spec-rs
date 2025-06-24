//! Console Device

use num_enum::{FromPrimitive, IntoPrimitive};
use volatile::access::ReadOnly;
use volatile_macro::VolatileFieldAccess;

pub use super::features::console::F;
use crate::{le16, le32};

/// Console Device Configuration Layout
///
/// Use [`ConfigVolatileFieldAccess`] to work with this struct.
#[doc(alias = "virtio_console_config")]
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
    cols: le16,
    #[access(ReadOnly)]
    rows: le16,
    #[access(ReadOnly)]
    max_nr_ports: le32,
    #[access(ReadOnly)]
    emerg_wr: le32,
}

/// Control Message
#[doc(alias = "virtio_console_control")]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy_derive::KnownLayout,
        zerocopy_derive::Immutable,
        zerocopy_derive::FromBytes,
        zerocopy_derive::IntoBytes,
    )
)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Control {
    /// Port number
    pub id: le32,
    /// The kind of control event
    pub event: le16,
    /// Extra information for the event
    pub value: le16,
}

/// Event
///
/// <div class="warning">
///
/// This enum is not ABI-compatible with it's corresponding field.
/// Use [`Device::from`] for converting from an integer.
///
/// </div>
///
/// [`Device::from`]: Device#impl-From<u16>-for-Device
#[doc(alias = "VIRTIO_CONSOLE")]
#[derive(IntoPrimitive, FromPrimitive, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u16)]
pub enum Device {
    /// Sent by the driver at initialization to indicate that it is ready to receive control messages.
    ///
    /// A value of 1 indicates success, and 0 indicates failure.
    /// The port number `id` is unused.
    #[doc(alias = "VIRTIO_CONSOLE_DEVICE_READY")]
    DeviceReady = 0,

    /// Sent by the device, to create a new port.
    ///
    /// `value` is unused.
    #[doc(alias = "VIRTIO_CONSOLE_DEVICE_ADD")]
    DeviceAdd = 1,

    /// Sent by the device, to remove an existing port.
    ///
    /// `value` is unused.
    #[doc(alias = "VIRTIO_CONSOLE_DEVICE_REMOVE")]
    DeviceRemove = 2,

    /// Sent by the driver in response to the device's VIRTIO_CONSOLE_PORT_ADD message, to indicate that the port is ready to be used.
    ///
    /// A `value` of 1 indicates success, and 0 indicates failure.
    #[doc(alias = "VIRTIO_CONSOLE_PORT_READY")]
    PortReady = 3,

    /// Sent by the device to nominate a port as a console port.
    ///
    /// There MAY be more than one console port.
    #[doc(alias = "VIRTIO_CONSOLE_CONSOLE_PORT")]
    ConsolePort = 4,

    /// Sent by the device to indicate a console size change.
    ///
    /// `value` is unused.
    /// The buffer is followed by the number of columns and rows ([`virtio_console_resize`]).
    ///
    /// [`virtio_console_resize`]: Resize
    #[doc(alias = "VIRTIO_CONSOLE_RESIZE")]
    Resize = 5,

    /// This message is sent by both the device and the driver.
    ///
    /// `value` indicates the state: 0 (port closed) or 1 (port open).
    /// This allows for ports to be used directly by guest and host processes to communicate in an application-defined manner.
    #[doc(alias = "VIRTIO_CONSOLE_PORT_OPEN")]
    PortOpen = 6,

    /// Sent by the device to give a tag to the port.
    ///
    /// This control command is immediately followed by the UTF-8 name of the port for identification within the guest (without a NUL terminator).
    #[doc(alias = "VIRTIO_CONSOLE_PORT_NAME")]
    PortName = 7,

    #[num_enum(catch_all)]
    Unknown(u16),
}

/// Resize Message Layout
#[doc(alias = "virtio_console_resize")]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy_derive::KnownLayout,
        zerocopy_derive::Immutable,
        zerocopy_derive::FromBytes,
        zerocopy_derive::IntoBytes,
    )
)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Resize {
    pub cols: le16,
    pub rows: le16,
}
