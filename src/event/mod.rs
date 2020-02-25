mod event;
mod attributes;
mod data;
mod spec_version;
mod extensions;

pub use event::Event;
pub use attributes::Attributes;
pub(crate) use attributes::{AttributesReader, AttributesWriter};
pub use data::Data;
pub(crate) use data::{DataDecoder, DataEncoder};
pub use spec_version::SpecVersion;
pub use extensions::ExtensionValue;

mod v10;

pub use v10::Attributes as AttributesV10;
