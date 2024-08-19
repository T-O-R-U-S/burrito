/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
extern crate core;


pub mod database;
pub mod providers;
pub mod signing;
pub mod encryption;

#[cfg(test)]
mod tests {
    use crate::database::Metadata;
    use crate::encryption::{EncryptionProvider, EncryptionProviderSymmetric};
    use crate::providers::burrito_box::BurritoBox;
    use crate::providers::burrito_box_sym::BurritoBoxSym;
    use crate::providers::sensitive_text::SensitiveText;
    use crate::providers::Provider;
    use crate::signing::Signing;
    use bson::Bson;
    use dryoc::dryocbox::protected::SecretKey;
    use dryoc::types::NewBytes;

    fn blank_key() -> SecretKey {
        SecretKey::new_bytes()
    }


    #[test]
    fn sym_sign_test() {
        let plaintext = SensitiveText::new("Hello World!");
        let secret_key = blank_key();

        let encrypted = BurritoBox::encrypt(plaintext.into_entry(), secret_key).expect("Failed to encrypt");

        let secret_key = blank_key();
        let encrypted = encrypted.into_entry().sign_sym(secret_key);

        let secret_key = blank_key();
        let mut verify = encrypted.verify_sym(secret_key).expect("Failed to verify signature");

        let x = verify.get_mut("ENCRYPTED").expect("Failed to get data");
        let Bson::Binary(b) = x else { panic!("Failed to get binary") };
        b.bytes.swap_remove(12); // ...oopsie! Our signature is invalid now!

        let secret_key = blank_key();
        let _err = verify.verify_sym(secret_key).expect_err("Signature should not be valid!");
    }

    #[test]
    fn sign_test() {
        use dryoc::sign::SigningKeyPair;
        use dryoc::sign::protected::PublicKey;
        use dryoc::sign::protected::SecretKey;

        let keypair = SigningKeyPair::<PublicKey, SecretKey>::gen();

        let plaintext = SensitiveText::new("Hello World!");

        let plaintext = plaintext.sign(keypair.secret_key.clone());

        let plaintext = plaintext.verify().expect("Signature must be correct.");

        let plaintext = plaintext.with_security(keypair.secret_key.clone());

        assert!(plaintext.is_secure());
    }

    #[test]
    fn encrypt_decrypt_test() {
        use dryoc::keypair::KeyPair;
        use dryoc::dryocbox::protected::SecretKey;
        use dryoc::dryocbox::protected::PublicKey;

        let keypair: KeyPair<PublicKey, SecretKey> = KeyPair::gen();

        let public_key = &keypair.public_key;
        let secret_key = &keypair.secret_key;

        let plaintext = SensitiveText::new("Hello World!").into_entry().sign_sym(public_key.clone());
        let encrypted = BurritoBox::encrypt(plaintext, public_key.clone()).expect("Failed to encrypt");

        println!("{:#}", bson::to_bson(&encrypted).unwrap());

        let decrypted = encrypted.decrypt(secret_key.clone()).expect("Failed to decrypt");
        println!("{:#}", bson::to_bson(&decrypted).unwrap());
    }

    #[test]
    fn security_attestation_test() {
        use dryoc::sign::SigningKeyPair;
        use dryoc::sign::protected::PublicKey;
        use dryoc::sign::protected::SecretKey;

        let plaintext = SensitiveText::new("Hello World!");
        let keypair = SigningKeyPair::<PublicKey, SecretKey>::gen();
        let plaintext = plaintext.with_security(keypair.secret_key.clone());
        let is_secure = plaintext.is_secure();

        assert!(is_secure);

        let plaintext = plaintext.sign(keypair.secret_key.clone());

        let plaintext = plaintext.verify().expect("Signature must be correct.");

        // By adding a signature, we actually modified the document store, so it should not be secure!
        // This is by design, to prevent accidentally adding unintended extra data to a security-authorized
        // document.
        assert!(!plaintext.is_secure());
    }

    #[test]
    fn encrypt_decrypt_sym_test() {
        let plaintext = SensitiveText::new("Hello World!");
        let secret_box = BurritoBoxSym::encrypt_sym(plaintext.into_entry(), blank_key()).expect("Failed to encrypt");

        println!("{:#}", bson::to_bson(&secret_box).unwrap());
    }

    #[test]
    fn serialize_test() {
        let plaintext = SensitiveText::new("Hello World!").and_defaults::<SensitiveText>().sign_sym(blank_key());
        println!("{:#}", bson::to_bson(&plaintext).unwrap());
        let verified = plaintext.verify_sym(blank_key()).expect("Failed to verify signature");
        println!("{:#}", bson::to_bson(&verified).unwrap());

        let entry = verified.into_entry();
        println!("{:#}", bson::to_bson(&entry).unwrap());
        let plaintext = SensitiveText::from_entry(entry).expect("Failed to verify signature");
        let _plaintext = plaintext.verify_sym(blank_key()).expect("Failed to verify signature");
    }
}