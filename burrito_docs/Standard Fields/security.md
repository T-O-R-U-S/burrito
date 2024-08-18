# Security

Inevitably, data will have to be transported over the network, or stored where anyone can later access it. This is very
useful to users, but it also presents a security risk to your data.

The burrito schema is designed to address these concerns.

Encryption itself comes with too many specifics to be usable in standard fields. Instead, Burrito comes with two providers
that are used for encrypted data -- `burrito_asymmetric_box` (Public Key/Private Key) and `burrito_symmetric_box` (Secret Key).

You can use these providers to encrypt your data and then store it in the database.

## Prevent tampering

#### ---------------------
#### Forewarning: The best way to prevent tampering is to encrypt your data.
#### ---------------------

The fields below are designed to prevent unauthorized modification of database entries.

### Symmetric

- `sym_signature`: Entries can be signed with a secret key to prevent tampering, and verified only with the signing key.

### Asymmetric

1. `signature`: Entries can be signed with a secret key to prevent tampering, and verified with a public key. 

- Any attacker can remove this field, and the entry will still be valid. 
- Reject any entry that does not have a `signature` field if you expect the entry to be signed.

2. `signing_public_key`: Entries can be signed with a public key to prevent tampering, and verified with a public key.

- This contains the public key that was used to sign the entry.
- This field is optional if you already have the public key.
- Only trust and accept public keys that you have recognized and manually approved.

3. `assumed_secure`: Entries can be signed to attest that the owner approves the entry to be handled insecurely.

- This field is a signature.
- If this field is present and signed with a trusted public key, then the entry is safe to write to disk, or transport 
  over the network.
- A potential mistake is to add a security attestation and a signature to the same entry. Not only is this redundant,
  but it also invalidates one of your signatures, as you've now changed the document. Use only one.

4. `security_signing_public_key`: Public key to use with `assumed_secure`

- The public key used with the `assumed_secure` field.
- Use this to verify the authenticity of the `assumed_secure` field.
- Use ONLY trusted public keys. Reject any public key that you do not know or trust.
- This field is optional if you already have the public key.
- A potential mistake is to add a security attestation and a signature to the same entry. Not only is this redundant,
  but it also invalidates one of your signatures, as you've now changed the document. Use only one.