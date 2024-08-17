/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::AppendMetadata;
use anyhow::bail;
use bson::spec::BinarySubtype;
use bson::Bson;
use dryoc::protected::HeapByteArray;
use dryoc::sign::protected::{PublicKey, SecretKey};
use dryoc::auth::protected::Key;
use serde::Serialize;

pub trait Signing: AppendMetadata + Serialize {
    /// symmetrical signature
    fn sign_sym(self, secret_key: Key) -> Self
    {
        use dryoc::auth::Auth;
        use dryoc::constants::CRYPTO_AUTH_BYTES;

        // If the entries are not always in the same order, the signature will be different!!!!
        // I used a BTreeMap internally to make sure the entries are always in the same order
        let self_bytes = bson::to_vec(&self).expect("Failed to serialize entry");
        let signature: [u8; CRYPTO_AUTH_BYTES] = Auth::compute(secret_key, &self_bytes);
        let signature = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: Vec::from(signature),
        };

        self
            .append_meta(("signature_sym", signature))
    }

    fn verify_sym(self, secret_key: Key) -> anyhow::Result<Self>
    {
        use dryoc::auth::Auth;

        let mut self_entries = bson::to_document(&self)?;
        let Some(Bson::Binary(signature)) = self_entries.remove("signature_sym") else { bail!("Entry does not contain valid signature") };
        let signature = signature.bytes;
        // If the entries are not always in the same order, the signature will be different!!!!
        // I used a BTreeMap to make sure the entries are always in the same order
        let self_bytes = bson::to_vec(&self_entries).expect("Failed to serialize entry");
        Auth::compute_and_verify(&signature.as_slice(), secret_key, &self_bytes)?;

        Ok(self)
    }

    fn sign(self, key: SecretKey) -> Self {
        use dryoc::sign::SigningKeyPair;
        use dryoc::sign::protected::PublicKey;
        use dryoc::sign::protected::SecretKey;

        let keypair: SigningKeyPair<PublicKey, SecretKey> = SigningKeyPair::from_secret_key(key);
        let self_bytes = bson::to_vec(&self).expect("Failed to serialize entry");
        let (signature, _data): (HeapByteArray<64>, _) = keypair.sign(self_bytes).expect("Failed to sign entry").into_parts();
        let signature = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: signature.to_vec(),
        };

        self.append_meta(("signature", signature))
    }

    fn verify(self, key: PublicKey) -> anyhow::Result<Self> {
        use dryoc::sign::SignedMessage;

        let mut self_entries = bson::to_document(&self)?;
        let Some(Bson::Binary(signature)) = self_entries.remove("signature") else { bail!("Entry does not contain valid signature") };
        let signature = signature.bytes;
        let self_signed = bson::to_vec(&self_entries)?;
        let self_signed = SignedMessage::from_parts(signature, self_signed);
        self_signed.verify(&key)?;

        Ok(self)
    }

    fn with_security(self, key: SecretKey) -> Self {
        use dryoc::sign::SigningKeyPair;
        use dryoc::sign::protected::PublicKey;
        use dryoc::sign::protected::SecretKey;

        let keypair: SigningKeyPair<PublicKey, SecretKey> = SigningKeyPair::from_secret_key(key);
        let self_bytes = bson::to_vec(&self).expect("Failed to serialize entry");
        let (signature, _data): (HeapByteArray<64>, _) = keypair.sign(self_bytes).expect("Failed to sign entry").into_parts();
        let signature = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: signature.to_vec(),
        };

        self.append_meta(("assumed_secure", signature))
    }

    fn is_secure(&self, public_key: PublicKey) -> bool {
        todo!()
    }
}

impl<T: AppendMetadata + Serialize> Signing for T {}