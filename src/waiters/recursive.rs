/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::{Entry, Metadata};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::waiters::Waiter;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

impl Waiter for Recursive {
    fn name() -> String {
        "burrito_recursive".to_string()
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

        let entry = bson::from_document(entry)?;

        Ok(entry)
    }
}

impl Metadata for Recursive {
    fn set_meta(&mut self, metadata: (&str, impl Serialize)) {
        self.additional_fields.insert(metadata.0.to_string(), bson::to_bson(&metadata.1).unwrap());
    }

    fn get_meta(&self, key: &str) -> Option<&bson::Bson> {
        self.additional_fields.get(key)
    }
}