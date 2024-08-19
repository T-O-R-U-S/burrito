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
    fn get_meta(&self, key: &str) -> Option<&bson::Bson>;

    fn write_meta(&mut self, metadata: (&str, impl Serialize));

    fn add_meta(&mut self, metadata: (&str, impl Serialize)) {
        if self.get_meta(metadata.0).is_none() {
            self.write_meta(metadata);
        }
    }

    fn with_meta(mut self, metadata: (&str, impl Serialize)) -> Self {
        self.write_meta(metadata);
        self
    }

    fn with_added(mut self, metadata: (&str, impl Serialize)) -> Self {
        self.add_meta(metadata);
        self
    }

    fn and_defaults<T: Provider>(self) -> Self {
        self.with_meta(("provider", T::name()))
            .with_meta(("version", T::version()))
            .with_added(("created", bson::DateTime::now()))
    }
}

impl Metadata for Entry {
    fn get_meta(&self, key: &str) -> Option<&bson::Bson> {
        self.get(key)
    }

    fn write_meta(&mut self, metadata: (&str, impl Serialize)) {
        let (key, value) = metadata;
        let value = bson::to_bson(&value).expect("Failed to serialize metadata");
        self.insert(key, value);
    }
}

impl Metadata for BTreeMap<String, bson::Bson> {
    fn get_meta(&self, key: &str) -> Option<&bson::Bson> {
        self.get(key)
    }

    fn write_meta(&mut self, metadata: (&str, impl Serialize)) {
        self.insert(metadata.0.to_string(), bson::to_bson(&metadata.1).unwrap());
    }
}