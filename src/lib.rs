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
    use crate::database::{AppendMetadata, Provider};
    use crate::encryption::EncryptionProvider;
    use crate::providers::burrito_box::BurritoBox;
    use crate::providers::plaintext::Plaintext;
    use crate::signing::Signing;
    use bson::Bson;
    use dryoc::dryocbox::protected::SecretKey;
    use dryoc::types::NewBytes;

    fn blank_key() -> SecretKey {
        SecretKey::new_bytes()
    }

    #[test]
    fn sym_sign_test() {
        let plaintext = Plaintext::new("Hello World!");
        let secret_key = blank_key();

        let encrypted = BurritoBox::encrypt(plaintext.into_entry(), secret_key).expect("Failed to encrypt");

        let secret_key = blank_key();
        let encrypted = encrypted.into_entry().sign_sym(secret_key);

        let secret_key = blank_key();
        let mut verify = encrypted.verify_sym(secret_key).expect("Failed to verify signature");

        let x = verify.get_mut("encrypted_burrito_box").expect("Failed to get signature");
        let Bson::Binary(b) = x else { panic!("Failed to get binary") };
        b.bytes.swap_remove(12); // ..oopsie! Our signature is invalid now!

        let secret_key = blank_key();
        let _err = verify.verify_sym(secret_key).expect_err("Signature should not be valid!");
    }

    #[test]
    fn encrypt_decrypt_test() {
        use dryoc::keypair::KeyPair;
        use dryoc::dryocbox::protected::SecretKey;
        use dryoc::dryocbox::protected::PublicKey;

        let keypair: KeyPair<PublicKey, SecretKey> = KeyPair::gen();

        let public_key = &keypair.public_key;
        let secret_key = &keypair.secret_key;

        let plaintext = Plaintext::new("Hello World!").into_entry().sign_sym(public_key.clone());
        let encrypted = BurritoBox::encrypt(plaintext, public_key.clone()).expect("Failed to encrypt");

        println!("{}", bson::to_bson(&encrypted).unwrap());

        let decrypted = encrypted.decrypt(secret_key.clone()).expect("Failed to decrypt");
        println!("{}", bson::to_bson(&decrypted).unwrap());
    }

    #[test]
    fn serialize_test() {
        let plaintext = Plaintext::new("Hello World!").with_meta::<Plaintext>().sign_sym(blank_key());
        let verified = plaintext.verify_sym(blank_key()).expect("Failed to verify signature");

        let entry = verified.into_entry();
        let plaintext = Plaintext::from_entry(entry).expect("Failed to verify signature");
        let _plaintext = plaintext.verify_sym(blank_key()).expect("Failed to verify signature");
    }
}