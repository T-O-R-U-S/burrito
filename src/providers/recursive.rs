/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::{AppendMetadata, Entry, Provider};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct Recursive {
    pub children: Vec<Entry>,
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, bson::Bson>,
}

impl Recursive {
    pub fn new(children: Vec<Entry>) -> Self {
        Self { children, additional_fields: BTreeMap::new() }
    }
}

impl Provider for Recursive {
    fn name() -> String {
        "burrito_recursive".to_string()
    }

    fn version() -> String {
        "0.0.0".to_string()
    }

    fn into_entry(self) -> Entry {
        let entry = bson::to_document(&self).unwrap();

        entry.with_meta::<Self>()
    }

    fn from_entry(entry: Entry) -> anyhow::Result<Self> {
        let entry = bson::from_document(entry)?;

        Ok(entry)
    }
}