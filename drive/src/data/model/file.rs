use uuid::Uuid;
use md5::Digest;

pub enum Format {
    VBox
}

pub struct Metadata {
    id: Uuid,
    hash: Digest,
    format: Format,
}

pub struct File {
    metadata: Metadata,
    content: String
}
