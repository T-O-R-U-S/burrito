/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::{Entry, Metadata};
use crate::encryption::EncryptionProvider;
use bson::doc;
use bson::spec::BinarySubtype;
use dryoc::dryocbox::protected::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::providers::Provider;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BurritoBox {
    encrypted: bson::Binary,
    ephemeral_public_key: bson::Binary,
    mac: bson::Binary,
    #[serde(flatten)]
    additional_fields: BTreeMap<String, bson::Bson>,
}

impl BurritoBox {
    pub fn from_encrypted(encrypted: dryoc::dryocbox::VecBox) -> Self {
        let (mac, encrypted, Some(key)) = encrypted.into_parts() else {
            // we always expect an ephemeral public key to be encoded.
            panic!("encrypted box is non-standard");
        };

        let encrypted = bson::Binary {
            subtype: BinarySubtype::Encrypted,
            bytes: encrypted,
        };

        let mac = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: mac.to_vec(),
        };

        let ephemeral_public_key = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: key.to_vec(),
        };

        Self {
            encrypted,
            mac,
            ephemeral_public_key,
            additional_fields: BTreeMap::new(),
        }
            .and_defaults::<Self>()
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
        bson::to_document(&self).unwrap().and_defaults::<Self>()
    }

    fn from_entry(entry: Entry) -> anyhow::Result<Self> {
        Self::verify_version(&entry)?;

        let burrito_box = bson::from_document(entry)?;

        Ok(burrito_box)
    }
}

impl Metadata for BurritoBox {
    fn get_meta(&self, key: &str) -> Option<&bson::Bson> {
        self.additional_fields.get(key)
    }

    fn write_meta(&mut self, metadata: (&str, impl Serialize)) {
        self.additional_fields.insert(metadata.0.to_string(), bson::to_bson(&metadata.1).unwrap());
    }
}

impl EncryptionProvider for BurritoBox {
    fn encrypt(entry: Entry, key: PublicKey) -> anyhow::Result<Self> {
        use dryoc::dryocbox::VecBox;

        let entry_bytes = bson::to_vec(&entry)?;
        let secret_box = VecBox::seal(&entry_bytes, &key)?;

        Ok(Self::from_encrypted(secret_box))
    }

    fn decrypt(self, key: SecretKey) -> anyhow::Result<Entry> {
        use dryoc::dryocbox::VecBox;
        use dryoc::dryocbox::protected::SecretKey;
        use dryoc::dryocbox::PublicKey;
        use dryoc::dryocbox::Mac;
        use dryoc::keypair::KeyPair;

        let keypair: KeyPair<PublicKey, SecretKey> = KeyPair::from_secret_key(key);

        let mac = self.mac.bytes;
        let mac = Mac::try_from(mac.as_slice())?;

        let encrypted = self.encrypted.bytes;

        let ephemeral_public_key = self.ephemeral_public_key.bytes;
        let ephemeral_public_key = PublicKey::try_from(ephemeral_public_key.as_slice())?;

        let encrypted = VecBox::from_parts(mac, encrypted, Some(ephemeral_public_key));

        let unencrypted = encrypted.unseal_to_vec(&keypair)?;
        let unencrypted = bson::from_slice(&unencrypted)?;


        Ok(unencrypted)
    }
}