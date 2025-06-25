use crate::generated::MetadataSchema;

pub trait SourceFieldsMetadata {
    fn get_metadata(&self) -> MetadataSchema;
}
