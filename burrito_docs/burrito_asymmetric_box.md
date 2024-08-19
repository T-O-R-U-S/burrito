# Asymmetric Box

The burrito box is based on libsodium’s public-key authenticated encryption, also known as a box. This implementation 
uses X25519 for key derivation, the XSalsa20 stream cipher, and Poly1305 for message authentication.

You should use the asymmetric box when you need to:
- Exchange messages between computers or between people
- Authenticate documents
- Avoid exposing secrets to the public

> ###### ...you know, this burrito metaphor just keeps getting better and better. I mean, XSalsa!? Salsa?! That like... goes with burritos don't it!?

## Asymmetric Box Format

```json5
{
  "ENCRYPTED": 0x42, // ...binary data...
  "EPHEMERAL_PUBLIC_KEY": 0x42, // ...binary data... CHANGE THIS EVERY TIME YOU DECRYPT AND ENCRYPT THE DATA!
  "MAC": 0x42, // ...binary data...
  // standard fields:
  "provider": "burrito_asymmetric_box",
  "version": "0.0.0",
}
```