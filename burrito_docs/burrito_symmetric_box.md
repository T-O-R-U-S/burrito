# Burrito Symmetric Box

The burrito symmetric box is based on libsodium’s secret-key authenticated encryption, also known as a *secret*box. This
implementation uses the XSalsa20 stream cipher, and Poly1305 for message authentication.

## Burrito Symmetric Box Format

```json5
{
  "encrypted": 0x42, // ...binary data...
  "mac": 0x42, // ...binary data...
  "nonce": 0x42, // ...binary data...
  // standard fields:
  "provider": "burrito_symmetric_box",
  "version": "0.0.0",
}
```

## Explanation

- `encrypted` is the encrypted data (doy).
- `mac` is the message authentication code (MAC).
- `nonce` is the nonce used to encrypt the data. Change this every time you decrypt and encrypt the data.