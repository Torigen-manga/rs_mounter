use crate::generated::MetadataSchema;

pub trait SourceMetadataClient {
    fn get_metadata(&self) -> MetadataSchema;
}
