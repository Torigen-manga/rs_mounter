use crate::generated::MetadataSchema;

pub trait SourceMetadataProvider {
    fn get_metadata(&self) -> MetadataSchema;
}
