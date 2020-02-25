use super::{Attributes, Data, DataDecoder, DataEncoder, AttributesReader, AttributesWriter, SpecVersion, ExtensionValue};
use uriparse::URIReference;

pub struct Event {
    pub attributes: Attributes,
    pub data: Option<Data>
}

//TODO macro for this one?
impl AttributesReader for Event {
    fn get_id(&self) -> &str {
        self.attributes.get_id()
    }

    fn get_type(&self) -> &str {
        self.attributes.get_type()
    }

    fn get_source(&self) -> &URIReference {
        self.attributes.get_source()
    }

    fn get_specversion(&self) -> SpecVersion {
        self.attributes.get_specversion()
    }

    fn get_datacontenttype(&self) -> Option<&str> {
        self.attributes.get_datacontenttype()
    }

    fn get_extension(&self, extension_name: &str) -> Option<&ExtensionValue> {
        self.attributes.get_extension(extension_name)
    }
}

//TODO macro for this one?
impl AttributesWriter for Event {
    fn set_id<'event>(&'event mut self, id: impl Into<&'event str>) {
        self.attributes.set_id(id)
    }

    fn set_type<'event>(&'event mut self, ty: impl Into<&'event str>) {
        self.attributes.set_type(ty)
    }

    fn set_source<'event>(&'event mut self, source: impl Into<URIReference<'event>>) {
        self.attributes.set_source(source)
    }

    fn set_datacontenttype<'event>(&'event mut self, datacontenttype: Option<impl Into<&'event str>>) {
        self.attributes.set_datacontenttype(datacontenttype)
    }

    fn set_extension<'event>(&'event mut self, extension_name: &'event str, extension_value: impl Into<ExtensionValue>) {
        self.attributes.set_extension(extension_name, extension_value)
    }

    fn remove_extension<'event>(&'event mut self, extension_name: &'event str) -> bool {
        self.attributes.remove_extension(extension_name)
    }
}

pub trait DataWriter<T: Sized, E: std::error::Error, D: DataEncoder<T, E>>
    where
        Self: Sized,
{
    fn write_payload(&mut self, value: &T) -> Result<(), E>;
}

pub trait DataReader<T: Sized, E: std::error::Error, D: DataDecoder<T, E>> {
    fn read_data(&self) -> Option<Result<&T, E>>;
}

impl <T: Sized, E: std::error::Error, D: DataEncoder<T, E>> DataWriter<T, E, D> for Event {
    fn write_payload(&mut self, value: &T) -> Result<(), E> {
        self.data = Some(D::encode_data(value)?);
        Ok(())
    }
}

impl <T: Sized, E: std::error::Error, D: DataDecoder<T, E>> DataReader<T, E, D> for Event {
    fn read_data(&self) -> Option<Result<&T, E>> {
        self.data.as_ref().map(D::decode_data)
    }
}

