# Sensitive Text

The sensitive text is a simple wrapper around BSON's String type that stores unencrypted text.

Wrap your sensitive text in a `burrito_symmetric_box` or `burrito_asymmetric_box` to encrypt it and keep it safe.

Users of the burrito protocol should always keep `sensitive_text`s as secure as possible and avoid sharing them over the network,
or writing them to disk, because there is a good chance that a `sensitive_text` is a password or other sensitive information (d'oh...).

## Plaintext Format

```json5
{
  "PLAINTEXT": "Hello World!",
  // standard fields:
  "provider": "sensitive_text",
  "version": "0.0.0",
}
```