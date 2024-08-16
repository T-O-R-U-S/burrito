/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use bson::Bson;
use orion::kdf::SecretKey;

pub type Entry = bson::Document;


pub trait Provider: Sized {
    fn name() -> String;
    fn version() -> String;
    fn assumed_secure() -> bool;
    fn into_entry(self) -> Entry;
    fn from_entry(entry: Entry) -> anyhow::Result<Self>;
}

pub trait EncryptionProvider: Provider {
    fn encrypt(entry: Entry, key: SecretKey) -> anyhow::Result<Self>;
    fn decrypt(self, data: SecretKey) -> anyhow::Result<Entry>;
}

pub trait AppendMetadata: Sized {
    fn with_meta<T: Provider>(self) -> Self;
}

impl AppendMetadata for Bson {
    fn with_meta<T: Provider>(mut self) -> Self {
        let Some(doc) = self.as_document_mut() else {
            panic!("Entry is not a document!");
        };

        doc.insert("provider", T::name());
        doc.insert("version", T::version());
        doc.insert("assumed_secure", T::assumed_secure());

        self
    }
}

impl AppendMetadata for bson::Document {
    fn with_meta<T: Provider>(mut self) -> Self {
        self.insert("provider", T::name());
        self.insert("version", T::version());
        self.insert("assumed_secure", T::assumed_secure());

        self
    }
}