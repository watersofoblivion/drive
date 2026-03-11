use uuid::Uuid;

use super::super::model::{Metadata, File};

pub trait Files {
    fn find_all() -> Vec<Metadata>;
    fn content(id: Uuid) -> String;
    fn add(file: File) -> ();
    fn update_metadata(metadata: Metadata) -> ();
    fn update_content(id: Uuid, content: String) -> ();
    fn delete(id: Uuid) -> ();
}