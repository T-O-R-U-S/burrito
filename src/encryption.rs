/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use dryoc::dryocbox::protected::{PublicKey, SecretKey};
use crate::database::Entry;
use crate::providers::Provider;

pub trait EncryptionProvider: Provider {
    fn encrypt(entry: Entry, key: PublicKey) -> anyhow::Result<Self>;
    fn decrypt(self, key: SecretKey) -> anyhow::Result<Entry>;
}

pub trait EncryptionProviderSymmetric: Provider {
    fn encrypt_sym(entry: Entry, key: SecretKey) -> anyhow::Result<Self>;
    fn decrypt_sym(self, key: PublicKey) -> anyhow::Result<Entry>;
}