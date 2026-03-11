Local Data Store Backend
===

This package provides an implementation of the Data Store interface(s) that store data in a local database.  The particular database is a [`redb`](https://docs.rs/redb/latest/redb/) key/value store, by default stored in the user's `${HOME}` directory at `~/.drive/drive.db`.