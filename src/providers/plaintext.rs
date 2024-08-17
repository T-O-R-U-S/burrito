/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::{AppendMetadata, Entry, Provider};
use bson::doc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Plaintext {
    pub plaintext: String,
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, bson::Bson>,
}

impl Plaintext {
    pub fn new(plaintext: &str) -> Self {
        Self {
            plaintext: plaintext.to_string(),
            additional_fields: BTreeMap::new(),
        }
    }
}

impl Provider for Plaintext {
    fn name() -> String {
        "plaintext".to_string()
    }

    fn version() -> String {
        "0.0.0".to_string()
    }
    fn into_entry(self) -> Entry {
        let entry = bson::ser::to_document(&self).unwrap();

        entry.with_meta::<Self>()
    }

    fn from_entry(entry: Entry) -> anyhow::Result<Self> {
        let entry: Plaintext = bson::de::from_document(entry)?;

        Ok(entry)
    }
}

impl AppendMetadata for Plaintext {
    fn append_meta(mut self, metadata: (&str, impl Serialize)) -> Self {
        self.additional_fields.insert(metadata.0.to_string(), bson::to_bson(&metadata.1).unwrap());
        self
    }
}