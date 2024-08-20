/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use dryoc::dryocbox::protected::{PublicKey, SecretKey};
use crate::database::Entry;
use crate::waiters::Waiter;

pub trait EncryptionWaiter: Waiter {
    fn encrypt(entry: Entry, key: PublicKey) -> anyhow::Result<Self>;
    fn decrypt(self, key: SecretKey) -> anyhow::Result<Entry>;
}

pub trait EncryptionWaiterSymmetric: Waiter {
    fn encrypt_sym(entry: Entry, key: SecretKey) -> anyhow::Result<Self>;
    fn decrypt_sym(self, key: SecretKey) -> anyhow::Result<Entry>;
}

pub trait KeyFrom: Waiter {
    fn key_from(&self, data: Vec<u8>) -> anyhow::Result<SecretKey>;
}