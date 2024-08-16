/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
extern crate core;

pub mod database;
pub mod providers;

#[cfg(test)]
mod tests {
    use crate::providers::plaintext::Plaintext;
    use orion::kdf::SecretKey;

    #[test]
    fn kdf_test() {
        let plaintext = Plaintext::new("Hello World!");
        let secret_key = SecretKey::generate(128).expect("Failed to generate secret key");
    }
}