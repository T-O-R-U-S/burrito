/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use serde::Serialize;

pub type Entry = bson::Document;

// Sort entry keys lexicographically
pub fn to_sorted(entry: Entry) -> Entry {
    let mut entries: Vec<(String, bson::Bson)> = entry.into_iter().collect();
    entries.sort_by(|(a, _), (b, _)| a.cmp(b));
    let doc: Entry = entries.into_iter().collect();
    doc
}

pub trait Provider: Sized {
    fn name() -> String;
    fn version() -> String;
    fn into_entry(self) -> Entry;
    fn from_entry(entry: Entry) -> anyhow::Result<Self>;
}


pub trait AppendMetadata: Sized {
    fn append_meta(self, metadata: (&str, impl Serialize)) -> Self;

    fn with_meta<T: Provider>(self) -> Self {
        self.append_meta(("provider", T::name()))
            .append_meta(("version", T::version()))
    }
}

impl AppendMetadata for Entry {
    fn append_meta(mut self, metadata: (&str, impl Serialize)) -> Self {
        let (key, value) = metadata;
        let value = bson::to_bson(&value).expect("Failed to serialize metadata");
        self.insert(key, value);
        self
    }
}