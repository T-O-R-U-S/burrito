/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::{Entry, Metadata};
use crate::encryption::EncryptionWaiterSymmetric;
use crate::waiters::Waiter;
use bson::spec::BinarySubtype;
use dryoc::dryocbox::protected::SecretKey;
use dryoc::dryocsecretbox::Nonce;
use dryoc::types::NewByteArray;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BurritoBoxSym {
    pub encrypted: bson::Binary,
    pub mac: bson::Binary,
    pub nonce: bson::Binary,
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, bson::Bson>,
}

impl BurritoBoxSym {
    pub fn of_password(entry: Entry, mut password: String) -> Self {
        use dryoc::dryocbox::protected::SecretKey;
        use dryoc::dryocbox::protected::PublicKey;
        use dryoc::keypair::KeyPair;
        use dryoc::dryocsecretbox::VecBox;

        let nonce = Nonce::gen();
        let salt = nonce.to_vec();
        let config = dryoc::pwhash::Config::interactive();
        let keypair: KeyPair<PublicKey, SecretKey> = dryoc::pwhash::PwHash::derive_keypair(unsafe { password.as_mut_vec() }, salt, config).unwrap();
        let secret_key = &keypair.secret_key;

        let entry_bytes = bson::to_vec(&entry).unwrap();
        let encrypted = VecBox::encrypt(&entry_bytes, &nonce, secret_key);
        let (mac, encrypted) = encrypted.into_parts();

        let encrypted = bson::Binary {
            subtype: BinarySubtype::Encrypted,
            bytes: encrypted,
        };

        let mac = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: mac.to_vec(),
        };

        let nonce = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: nonce.to_vec(),
        };

        Self {
            encrypted,
            mac,
            nonce,
            additional_fields: BTreeMap::new(),
        }
            .and_defaults::<Self>()
    }

    pub fn decrypt_password(self, mut password: String) -> anyhow::Result<Entry> {
        use dryoc::dryocbox::protected::SecretKey;
        use dryoc::dryocbox::protected::PublicKey;
        use dryoc::keypair::KeyPair;
        use dryoc::dryocsecretbox::VecBox;
        use dryoc::dryocsecretbox::Mac;

        let salt = self.nonce.bytes;
        let nonce = Nonce::try_from(salt.as_slice())?;


        let config = dryoc::pwhash::Config::interactive();
        let keypair: KeyPair<PublicKey, SecretKey> = dryoc::pwhash::PwHash::derive_keypair(unsafe { password.as_mut_vec() }, salt, config)?;

        let mac = Mac::try_from(self.mac.bytes.as_slice())?;

        let encrypted = VecBox::from_parts(mac, self.encrypted.bytes);

        let decrypted = encrypted.decrypt_to_vec(&nonce, &keypair.secret_key)?;
        let entry = bson::from_slice(&decrypted)?;

        Ok(entry)
    }
}

impl Waiter for BurritoBoxSym {
    fn name() -> String {
        "burrito_symmetric_box".to_string()
    }

    fn version() -> String {
        "0.0.0".to_string()
    }

    fn into_entry(self) -> Entry {
        bson::to_document(&self)
            .unwrap()
            .and_defaults::<Self>()
    }

    fn from_entry(entry: Entry) -> anyhow::Result<Self> {
        Self::verify_version(&entry)?;

        let burrito = bson::from_document(entry)?;

        Ok(burrito)
    }
}

impl Metadata for BurritoBoxSym {
    fn get_meta(&self, key: &str) -> Option<&bson::Bson> {
        self.additional_fields.get(key)
    }

    fn set_meta(&mut self, metadata: (&str, impl Serialize)) {
        self.additional_fields.insert(metadata.0.to_string(), bson::to_bson(&metadata.1).unwrap());
    }
}

impl EncryptionWaiterSymmetric for BurritoBoxSym {
    fn encrypt_sym(entry: Entry, key: SecretKey) -> anyhow::Result<Self> {
        use dryoc::dryocsecretbox::VecBox;
        use dryoc::dryocsecretbox::Nonce;

        let entry_bytes = bson::to_vec(&entry)?;
        let nonce = Nonce::gen();
        let encrypted = VecBox::encrypt(&entry_bytes, &nonce, &key);
        let (mac, encrypted) = encrypted.into_parts();

        let encrypted = bson::Binary {
            subtype: BinarySubtype::Encrypted,
            bytes: encrypted,
        };

        let mac = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: mac.to_vec(),
        };

        let nonce = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: nonce.to_vec(),
        };

        Ok(
            Self {
                encrypted,
                mac,
                nonce,
                additional_fields: BTreeMap::new(),
            }
                .and_defaults::<Self>()
        )
    }

    fn decrypt_sym(self, key: SecretKey) -> anyhow::Result<Entry> {
        use dryoc::dryocsecretbox::VecBox;
        use dryoc::dryocsecretbox::Mac;

        let mac = Mac::try_from(self.mac.bytes.as_slice())?;
        let encrypted = self.encrypted.bytes;
        let nonce = self.nonce.bytes;

        let encrypted = VecBox::from_parts(mac, encrypted);
        let decrypted = encrypted.decrypt_to_vec(&nonce, &key)?;

        let entry = bson::from_slice(&decrypted)?;

        Ok(entry)
    }
}