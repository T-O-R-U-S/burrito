/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::{Entry, Metadata};
use bson::doc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::providers::Provider;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SensitiveText {
    pub plaintext: String,
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, bson::Bson>,
}

impl SensitiveText {
    pub fn new(plaintext: &str) -> Self {
        Self {
            plaintext: plaintext.to_string(),
            additional_fields: BTreeMap::new(),
        }
    }
}

impl Provider for SensitiveText {
    fn name() -> String {
        "sensitive_text".to_string()
    }

    fn version() -> String {
        "0.0.0".to_string()
    }
    fn into_entry(self) -> Entry {
        let entry = bson::ser::to_document(&self).unwrap();

        entry.and_defaults::<Self>()
    }

    fn from_entry(entry: Entry) -> anyhow::Result<Self> {
        Self::verify_version(&entry)?;

        let entry: SensitiveText = bson::de::from_document(entry)?;

        Ok(entry)
    }
}

impl Metadata for SensitiveText {
    fn get_meta(&self, key: &str) -> Option<&bson::Bson> {
        self.additional_fields.get(key)
    }

    fn write_meta(&mut self, metadata: (&str, impl Serialize)) {
        self.additional_fields.insert(metadata.0.to_string(), bson::to_bson(&metadata.1).unwrap());
    }
}