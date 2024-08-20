# Burrito Recursive

This provider simply contains a list of child entries. A combo deal, if you will!

## Example

```json5
{
  "CHILDREN": [
    {
      "PLAINTEXT": "I'm a child entry",
      // ... standard fields:
      "waiter": "sensitive_text",
      "version": "0.0.0"
    }
  ],
  // ... standard fields:
  "version": "0.0.0",
  "waiter": "burrito_recursive",
}
```

## Usage examples

### Encrypt many entries and store them all at once

```json5
{
  "CHILDREN": [
    {
      "PLAINTEXT": "I'm a child entry",
      // ... standard fields:
      "version": "0.0.0",
      "waiter": "sensitive_text",
    },
  ],
  // ... standard fields:
  "version": "0.0.0",
  "waiter": "recursive",
}
```

### Encrypt many entries and store them in a nested structure

You can wrap a `burrito_recursive` in a burrito_symmetric_box or burrito_asymmetric_box to encrypt it.

```json5
{
  "CHILDREN": [
    {
      "PLAINTEXT": "I'm a child entry",
      // ... standard fields:
      "version": "0.0.0",
      "waiter": "plaintext",
    },
  ],
  // ... standard fields:
  "version": "0.0.0",
  "waiter": "burrito_recursive",
}
```

Then we wrap it in a burrito_symmetric_box:

```json5
{
  "ENCRYPTED": 0x42, // ...binary data...
  "EPHEMERAL_PUBLIC_KEY": 0x42, // ...binary data...
  "MAC": 0x42, // ...binary data...
  // standard fields:
  "version": "0.0.0",
  "waiter": "burrito_asymmetric_box",
}
```

### Store a list of credentials for one provider