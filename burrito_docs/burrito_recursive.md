# Burrito Recursive

This provider simply contains a list of child entries. A combo deal, if you will!

## Example

```json5
{
  "CHILDREN": [
    {
      "PLAINTEXT": "I'm a child entry",
      // ... standard fields:
      "provider": "plaintext",
      "version": "0.0.0"
    }
  ],
  // ... standard fields:
  "provider": "burrito_recursive",
  "version": "0.0.0"
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
      "provider": "plaintext",
      "version": "0.0.0"
    },
  ],
  // ... standard fields:
  "provider": "burrito_recursive",
  "version": "0.0.0"
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
      "provider": "plaintext",
      "version": "0.0.0"
    },
  ],
  // ... standard fields:
  "provider": "burrito_recursive",
  "version": "0.0.0"
}
```

Then we wrap it in a burrito_symmetric_box:

```json5
{
  "ENCRYPTED": 0x42, // ...binary data...
  "EPHEMERAL_PUBLIC_KEY": 0x42, // ...binary data...
  "MAC": 0x42, // ...binary data...
  // standard fields:
  "provider": "burrito_asymmetric_box",
  "version": "0.0.0",
}
```

### Store a list of credentials for one provider