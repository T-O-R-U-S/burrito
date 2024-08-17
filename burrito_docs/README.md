# Documentation for the Burrito specification

## Introduction

Burrito is a specification for a simple, extensible, and sufficient format for credential storage.

I chose BSON not only because it is a widely-used (MongoDB) format, but also because it is schema-less, allowing for
easy extension.

That being said, of-course, applications cannot work on schemaless data! So the Burrito spec is more like a schema
for BSON!

The most basic Burrito document looks something like this:

```json5
{
  // ... provider-specific fields
  "provider": "entry_provider", // a provider is kind of like a schema specific to that document type
  "version": "0.0.0", // semver version
  // ... other fields
}
```

A more practical example might be the asymmetric burrito box:

```json5
{
  // provider-specific fields:
  "encrypted": 0x42, // ...binary data...
  "mac": 0x42, // ...binary data...
  "ephemeral_public_key": 0x42, // ...binary data...
  // standard fields:
  "provider": "burrito_asymmetric_box",
  "version": "0.0.0",
}
```

## Field ordering
<hr />

The fields at the top of the document are provider-specific, and can only be in the order specified by the provider.

Metadata fields (i.e, `provider`, `version`) are always ordered lexicographically by key. This is to ensure that the same document can be hashed/signed, and then
compared to other documents to determine if they are the same. Any other ordering is non-standard and incorrect.

The examples present in the documentation should have correct ordering.

Rust's `BTreeMap` collection maintains this ordering for you -- `BTreeMap` also does not need `std`, unlike `HashMap`.

## Standard fields
<hr /> 

These are obligatory and must be present in every burrito entry.

### `provider`
The name of the provider.

### `version`
Semver version

## Standard security fields
<hr />

These are optional

### `assumed_secure`
Does it have security features that make it safe for storing/sharing unencrypted?

This field is not a boolean -- it is actually a signature.

### `signature`
Signature of the document using an asymmetric key. Use this if you need to know where the document came from.

Any attacker can remove this field from the document, so make sure to reject any document that does not have this field if you are expecting it.

#### Sidenote: The best way to ensure attackers don't tamper with documents is to encrypt them.

### `signature_sym`
Signature of the document using a symmetric key. Use this if you need to know the document is yours and has not been tampered with.

Any attacker can remove this field from the document, so make sure to reject any document that does not have this field if you are expecting it.

#### Sidenote: The best way to ensure attackers don't tamper with documents is to encrypt them.