/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::{Entry, Metadata};
use crate::encryption::KeyFrom;
use crate::waiters::Waiter;
use dryoc::dryocbox::protected::SecretKey;
use dryoc::pwhash::Salt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BurritoBoxPassword {
    pub data: bson::Binary,
    pub salt: String,
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, bson::Bson>,
}

impl Waiter for BurritoBoxPassword {
    fn name() -> String {
        "burrito_box_password_sym".to_string()
    }

    fn version() -> String {
        "0.0.0".to_string()
    }

    fn into_entry(self) -> Entry {
        let entry = bson::to_document(&self).unwrap();

        entry.and_defaults::<Self>()
    }

    fn from_entry(entry: Entry) -> anyhow::Result<Self> {
        Self::verify_version(&entry)?;

        let burrito = bson::from_document(entry)?;

        Ok(burrito)
    }
}

impl Metadata for BurritoBoxPassword {
    fn get_meta(&self, key: &str) -> Option<&bson::Bson> {
        self.additional_fields.get(key)
    }

    fn write_meta(&mut self, metadata: (&str, impl Serialize)) {
        self.additional_fields.insert(metadata.0.to_string(), bson::to_bson(&metadata.1).unwrap());
    }
}

impl KeyFrom for BurritoBoxPassword {
    fn key_from(&self, data: Vec<u8>) -> anyhow::Result<SecretKey> {
        use dryoc::pwhash::VecPwHash;
        use dryoc::pwhash::Config;
        use dryoc::keypair::StackKeyPair;

        let config = Config::sensitive();
        let salt = Salt::from(self.salt.as_str());

        let hash: StackKeyPair = VecPwHash::derive_keypair(&data, salt, config)?;

        Ok(hash.secret_key.clone().into())
    }
}