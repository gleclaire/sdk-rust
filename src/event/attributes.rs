use uriparse::URIReference;
use super::SpecVersion;
use crate::event::{AttributesV10, ExtensionValue};

pub trait AttributesReader {
    fn get_id(&self) -> &str;
    fn get_type(&self) -> &str;
    fn get_source(&self) -> &URIReference;
    fn get_specversion(&self) -> SpecVersion;
    fn get_datacontenttype(&self) -> Option<&str>;
    fn get_extension(&self, extension_name: &str) -> Option<&ExtensionValue>;
}

pub trait AttributesWriter {
    fn set_id<'event>(&'event mut self, id: impl Into<&'event str>);
    fn set_type<'event>(&'event mut self, ty: impl Into<&'event str>);
    fn set_source<'event>(&'event mut self, source: impl Into<URIReference<'event>>);
    fn set_datacontenttype<'event>(&'event mut self, datacontenttype: Option<impl Into<&'event str>>);
    fn set_extension<'event>(&'event mut self, extension_name: &'event str, extension_value: impl Into<ExtensionValue>);
    fn remove_extension<'event>(&'event mut self, extension_name: &'event str) -> bool;
}

pub enum Attributes {
    V10(AttributesV10)
}

// TODO maybe this one could be "macroed"?
impl AttributesReader for Attributes {
    fn get_id(&self) -> &str {
        match self {
            Attributes::V10(a) => a.get_id()
        }
    }

    fn get_type(&self) -> &str {
        match self {
            Attributes::V10(a) => a.get_type()
        }
    }

    fn get_source(&self) -> &URIReference {
        match self {
            Attributes::V10(a) => a.get_source()
        }
    }

    fn get_specversion(&self) -> SpecVersion {
        match self {
            Attributes::V10(a) => a.get_specversion()
        }
    }

    fn get_datacontenttype(&self) -> Option<&str> {
        match self {
            Attributes::V10(a) => a.get_datacontenttype()
        }
    }

    fn get_extension(&self, extension_name: &str) -> Option<&ExtensionValue> {
        match self {
            Attributes::V10(a) => a.get_extension(extension_name)
        }
    }
}

// TODO maybe this one could be "macroed"?
impl AttributesWriter for Attributes {
    fn set_id<'event>(&'event mut self, id: impl Into<&'event str>) {
        match self {
            Attributes::V10(a) => a.set_id(id)
        }
    }

    fn set_type<'event>(&'event mut self, ty: impl Into<&'event str>) {
        match self {
            Attributes::V10(a) => a.set_type(ty)
        }
    }

    fn set_source<'event>(&'event mut self, source: impl Into<URIReference<'event>>) {
        match self {
            Attributes::V10(a) => a.set_source(source)
        }
    }

    fn set_datacontenttype<'event>(&'event mut self, datacontenttype: Option<impl Into<&'event str>>) {
        match self {
            Attributes::V10(a) => a.set_datacontenttype(datacontenttype)
        }
    }

    fn set_extension<'event>(&'event mut self, extension_name: &'event str, extension_value: impl Into<ExtensionValue>) {
        match self {
            Attributes::V10(a) => a.set_extension(extension_name, extension_value)
        }
    }

    fn remove_extension<'event>(&'event mut self, extension_name: &'event str) -> bool {
        match self {
            Attributes::V10(a) => a.remove_extension(extension_name)
        }
    }
}
