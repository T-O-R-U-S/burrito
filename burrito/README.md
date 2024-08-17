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
  "provider": "entry_provider", // a provider is kind of like a schema specific to that document type
  "assumed_secure": false, // if false, then you should assume that the credential is not encrypted at all and should not be shared over the network.
  "version": "0.0.0", // semver version
  // ... other fields
}
```

A more practical example might be the asymmetric burrito box:

```json5
{
  "provider": "burrito_asymmetric_box",
  "assumed_secure": true,
  "version": "0.0.0",
  // custom fields:
  "encrypted_data": 0x42, // ... binary data...
}
```

## Standard fields

You've already been introduced to the most important fields:

- `provider`: the name of the provider.
- `assumed_secure`: does it have security features that make it safe for storing/sharing unencrypted?
- `version`: semver version
