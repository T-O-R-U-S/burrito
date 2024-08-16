/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::{AppendMetadata, Entry, Provider};
use bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Plaintext(
    #[serde(rename = "plaintext")]
    String
);

impl Plaintext {
    pub fn new(plaintext: &str) -> Self {
        Self(plaintext.to_string())
    }
}

impl Provider for Plaintext {
    fn name() -> String {
        "plaintext".to_string()
    }

    fn version() -> String {
        "0.0.0".to_string()
    }

    fn assumed_secure() -> bool {
        false
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