/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::{AppendMetadata, Entry, Provider};
use bson::doc;
use bson::spec::BinarySubtype;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use dryoc::dryocbox::protected::{PublicKey, SecretKey};
use crate::encryption::EncryptionProvider;

#[derive(Serialize, Deserialize)]
pub struct BurritoBox {
    encrypted_burrito_box: bson::Binary,
    #[serde(flatten)]
    additional_fields: BTreeMap<String, bson::Bson>,
}

impl BurritoBox {
    pub fn from_encrypted(encrypted_burrito_box: impl Into<bson::Binary>) -> Self {
        Self { encrypted_burrito_box: encrypted_burrito_box.into(), additional_fields: BTreeMap::new() }.with_meta::<Self>()
    }
}

impl Provider for BurritoBox {
    fn name() -> String {
        "burrito_asymmetric_box".to_string()
    }

    fn version() -> String {
        "0.0.0".to_string()
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

impl AppendMetadata for BurritoBox {
    fn append_meta(mut self, metadata: (&str, impl Serialize)) -> Self {
        self.additional_fields.insert(metadata.0.to_string(), bson::to_bson(&metadata.1).unwrap());

        self
    }
}

impl EncryptionProvider for BurritoBox {
    fn encrypt(entry: Entry, key: PublicKey) -> anyhow::Result<Self> {
        use dryoc::dryocbox::VecBox;

        let entry_bytes = bson::to_vec(&entry)?;
        let secret_box = VecBox::seal(&entry_bytes, &key)?;

        let encrypted = bson::Binary {
            subtype: BinarySubtype::Encrypted,
            bytes: secret_box.to_vec(),
        };

        Ok(Self::from_encrypted(encrypted))
    }

    fn decrypt(self, key: SecretKey) -> anyhow::Result<Entry> {
        use dryoc::dryocbox::VecBox;
        use dryoc::dryocbox::protected::SecretKey;
        use dryoc::dryocbox::protected::PublicKey;
        use dryoc::keypair::KeyPair;

        let keypair: KeyPair<PublicKey, SecretKey> = KeyPair::from_secret_key(key);

        let encrypted = VecBox::from_sealed_bytes(&self.encrypted_burrito_box.bytes)?;
        let unencrypted = encrypted.unseal_to_vec(&keypair)?;
        let unencrypted = bson::from_slice(&unencrypted)?;

        Ok(unencrypted)
    }
}