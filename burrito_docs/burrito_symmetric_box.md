# Burrito Symmetric Box

The burrito symmetric box is based on libsodium’s secret-key authenticated encryption, also known as a *secret*box. This
implementation uses the XSalsa20 stream cipher, and Poly1305 for message authentication.

> ###### ...you know, this burrito metaphor just keeps getting better and better. I mean, XSalsa!? Salsa?! That like... goes with burritos don't it!?

## Burrito Symmetric Box Format

```json5
{
  "ENCRYPTED": 0x42, // ...binary data...
  "MAC": 0x42, // ...binary data...
  "NONCE": 0x42, // ...binary data... CHANGE THIS EVERY TIME YOU DECRYPT AND ENCRYPT THE DATA!
  // standard fields:
  "waiter": "burrito_symmetric_box",
  "version": "0.0.0",
}
```

## Explanation

- `encrypted` is the encrypted data (d'oh...).
- `mac` is the message authentication code (MAC).
- `nonce` is the nonce used to encrypt the data. Change this every time you decrypt and encrypt the data.