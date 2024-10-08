/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::Metadata;
use anyhow::bail;
use bson::spec::BinarySubtype;
use bson::Bson;
use dryoc::auth::protected::Key;
use dryoc::protected::HeapByteArray;
use dryoc::sign::protected::{PublicKey, SecretKey};
use serde::Serialize;

pub trait Signing: Metadata + Serialize {
    /// symmetrical signature
    fn sign_sym(self, secret_key: Key) -> Self
    {
        use dryoc::auth::Auth;
        use dryoc::constants::CRYPTO_AUTH_BYTES;

        let out = self.with_meta(("modified", bson::DateTime::now()));

        // If the entries are not always in the same order, the signature will be different!!!!
        // I used a BTreeMap internally to make sure the entries are always in the same order
        let self_bytes = bson::to_vec(&out).expect("Failed to serialize entry");
        let signature: [u8; CRYPTO_AUTH_BYTES] = Auth::compute(secret_key, &self_bytes);
        let signature = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: Vec::from(signature),
        };

        out
            .with_meta(("signature_sym", signature))
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
        let public_key = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: keypair.public_key.to_vec(),
        };

        let out = self
            .with_meta(("modified", bson::DateTime::now()))
            .with_meta(("signing_public_key", public_key));

        let self_bytes = bson::to_vec(&out).expect("Failed to serialize entry");
        let (signature, _data): (HeapByteArray<64>, _) = keypair.sign(self_bytes).expect("Failed to sign entry").into_parts();
        let signature = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: signature.to_vec(),
        };


        out
            .with_meta(("signature", signature))
    }

    fn verify(self) -> anyhow::Result<Self> {
        let self_entries = bson::to_document(&self)?;

        let Some(Bson::Binary(public_key)) = self_entries.get_meta("signing_public_key") else { bail!("Entry does not contain valid signature") };
        let public_key = &public_key.bytes;
        let public_key = PublicKey::try_from(public_key.as_slice())?;

        self.verify_with(public_key)
    }

    fn verify_with(self, public_key: PublicKey) -> anyhow::Result<Self> {
        use dryoc::sign::SignedMessage;

        let mut self_entries = bson::to_document(&self)?;

        let Some(Bson::Binary(signature)) = self_entries.remove("signature") else { bail!("Entry does not contain valid signature") };
        let signature = signature.bytes;


        let self_signed = bson::to_vec(&self_entries)?;
        let self_signed = SignedMessage::from_parts(signature, self_signed);

        self_signed.verify(&public_key)?;

        Ok(self)
    }

    const SECURITY_PADDING: &'static [u8] = b"This is some extra data to ensure that the signature is different, instead of being simply copy-pastable if the owner of the document did not also sign the document BEFORE adding a security attestation.";

    fn with_security(self, key: SecretKey) -> Self {
        use dryoc::sign::SigningKeyPair;
        use dryoc::sign::protected::PublicKey;
        use dryoc::sign::protected::SecretKey;

        let keypair: SigningKeyPair<PublicKey, SecretKey> = SigningKeyPair::from_secret_key(key);
        let public_key = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: keypair.public_key.to_vec(),
        };

        let out = self
            .with_meta(("modified", bson::DateTime::now()))
            .with_meta(("security_signing_public_key", public_key));

        let mut self_bytes = bson::to_vec(&out).expect("Failed to serialize entry");
        self_bytes.extend_from_slice(Self::SECURITY_PADDING);
        let (signature, _data): (HeapByteArray<64>, _) = keypair.sign(self_bytes).expect("Failed to sign entry").into_parts();
        let signature = bson::Binary {
            subtype: BinarySubtype::Sensitive,
            bytes: signature.to_vec(),
        };


        out
            .with_meta(("assumed_secure", signature))
    }

    fn is_secure(&self) -> bool {
        let Ok(self_entries) = bson::to_document(&self) else { return false };
        let Some(Bson::Binary(public_key)) = self_entries.get("security_signing_public_key") else { return false };
        let public_key = &public_key.bytes;
        let Ok(public_key) = PublicKey::try_from(public_key.as_slice()) else { return false };

        self.is_secure_with(public_key)
    }

    fn is_secure_with(&self, public_key: PublicKey) -> bool {
        use dryoc::sign::SignedMessage;

        let Ok(mut self_entries) = bson::to_document(&self) else { return false };
        let Some(Bson::Binary(signature)) = self_entries.remove("assumed_secure") else { return false };
        let signature = signature.bytes;

        let Ok(mut self_bytes) = bson::to_vec(&self_entries) else { return false };
        self_bytes.extend_from_slice(Self::SECURITY_PADDING);
        let self_signed = SignedMessage::from_parts(signature, self_bytes);

        let Ok(()) = self_signed.verify(&public_key) else { return false };

        true
    }
}

impl<T: Metadata + Serialize> Signing for T {}