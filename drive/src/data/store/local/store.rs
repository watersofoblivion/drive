use std::path::Path;

use super::Error;

pub struct Store {}

impl Store {
    pub fn create(path: impl AsRef<Path>) -> Result<Self, Error> {
        let store = Store {};
        Ok(store)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let store = Store {};
        Ok(store)
    }
}
