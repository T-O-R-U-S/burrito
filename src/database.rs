/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use std::collections::BTreeMap;
use serde::Serialize;
use crate::providers::Provider;

pub type Entry = bson::Document;

pub trait Metadata: Sized {
    fn append_meta(self, metadata: (&str, impl Serialize)) -> Self;

    fn get_meta(&self, key: &str) -> Option<&bson::Bson>;

    fn with_meta<T: Provider>(self) -> Self {
        self.append_meta(("provider", T::name()))
            .append_meta(("version", T::version()))
    }
}

impl Metadata for Entry {
    fn append_meta(mut self, metadata: (&str, impl Serialize)) -> Self {
        let (key, value) = metadata;
        let value = bson::to_bson(&value).expect("Failed to serialize metadata");
        self.insert(key, value);
        self
    }

    fn get_meta(&self, key: &str) -> Option<&bson::Bson> {
        self.get(key)
    }
}

impl Metadata for BTreeMap<String, bson::Bson> {
    fn append_meta(mut self, metadata: (&str, impl Serialize)) -> Self {
        self.insert(metadata.0.to_string(), bson::to_bson(&metadata.1).unwrap());
        self
    }

    fn get_meta(&self, key: &str) -> Option<&bson::Bson> {
        self.get(key)
    }
}