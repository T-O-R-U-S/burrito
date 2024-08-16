/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::{AppendMetadata, EncryptionProvider, Entry, Provider};
use bson::doc;
use orion::aead::SecretKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BurritoBox {
    encrypted: Vec<u8>,
}

impl Provider for BurritoBox {
    fn name() -> String {
        "burrito_secret_box".to_string()
    }

    fn version() -> String {
        "0.0.0".to_string()
    }

    fn assumed_secure() -> bool {
        true
    }

    fn into_entry(self) -> Entry {
        let entry = bson::ser::to_document(&self).unwrap();

        entry.with_meta::<Self>()
    }

    fn from_entry(entry: Entry) -> anyhow::Result<Self> {
        let entry = bson::de::from_document(entry)?;

        Ok(entry)
    }
}

impl EncryptionProvider for BurritoBox {
    fn encrypt(entry: Entry, key: SecretKey) -> anyhow::Result<Self> {
        let encrypted = orion::aead::seal(&key, bson::ser::to_vec(&entry)?.as_slice())?;

        Ok(Self {
            encrypted
        })
    }

    fn decrypt(self, key: SecretKey) -> anyhow::Result<Entry> {
        let unencrypted = orion::aead::open(&key, self.encrypted.as_slice())?;
        let unencrypted = bson::from_slice(&unencrypted)?;

        Ok(unencrypted)
    }
}