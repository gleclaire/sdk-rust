use crate::event::{SpecVersion, ExtensionValue, AttributesReader, AttributesWriter};
use uriparse::URIReference;

pub struct Attributes {

}

impl AttributesReader for Attributes {
    fn get_id(&self) -> &str {
        unimplemented!()
    }

    fn get_type(&self) -> &str {
        unimplemented!()
    }

    fn get_source(&self) -> &URIReference {
        unimplemented!()
    }

    fn get_specversion(&self) -> SpecVersion {
        unimplemented!()
    }

    fn get_datacontenttype(&self) -> Option<&str> {
        unimplemented!()
    }

    fn get_extension(&self, extension_name: &str) -> Option<&ExtensionValue> {
        unimplemented!()
    }
}

impl AttributesWriter for Attributes {
    fn set_id<'event>(&'event mut self, id: impl Into<&'event str>) {
        unimplemented!()
    }

    fn set_type<'event>(&'event mut self, ty: impl Into<&'event str>) {
        unimplemented!()
    }

    fn set_source<'event>(&'event mut self, source: impl Into<URIReference<'event>>) {
        unimplemented!()
    }

    fn set_datacontenttype<'event>(&'event mut self, datacontenttype: Option<impl Into<&'event str>>) {
        unimplemented!()
    }

    fn set_extension<'event>(&'event mut self, extension_name: &'event str, extension_value: impl Into<ExtensionValue>) {
        unimplemented!()
    }

    fn remove_extension<'event>(&'event mut self, extension_name: &'event str) -> bool {
        unimplemented!()
    }
}
