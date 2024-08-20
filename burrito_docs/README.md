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
  // ... waiter-specific fields
  "waiter": "entry_waiter", // a waiter is kind of like a schema specific to that document type
  "version": "0.0.0", // semver version
  // ... other fields
}
```

###### Note: burrito files are actually represented in [BSON](https://bsonspec.org), but they are in JSON here for demonstration purposes.

## Waiters

A "waiter" is a way of processing the data inside the burrito. It tells an application how to understand the data.

A waiter is used to serve requests.

```json5
{
  "PLAINTEXT": "Credit Card Number: 4242-4242-4242-4242",
  // standard fields:
  "version": "0.0.0",
  "waiter": "sensitive_text",
}
```

## A more practical example

The asymmetrically encrypted burrito box:

```json5
{
  // waiter-specific fields:
  "ENCRYPTED": 0x42, // ...binary data...
  "EPHEMERAL_PUBLIC_KEY": 0x42, // ...binary data...
  "MAC": 0x42, // ...binary data...
  // standard fields:
  "version": "0.0.0",
  "waiter": "burrito_asymmetric_box",
}
```

## Field ordering
<hr />

The fields are ordered lexicographically by key using the UTF-8 value of each character. This is to ensure that the same
document can be hashed/signed, and then compared to other documents to determine if they are the same. Any other 
ordering is non-standard and will produce incorrect hashes/signatures.

Waiters should always use `SCREAMING_SNAKE_CASE` for their unique fields to avoid name collisions and to ensure
that their fields are always at the top of the document, with metadata fields at the bottom.

Using a B-Tree Map is the best way to ensure that the ordering is correct and performant.

## Standard fields
<hr />

The standard fields are reserved fields that should not be used by waiters.

You can read more  about the standard fields in the [standard_fields](Standard%20Fields) directory.

## Standard implementations
<hr />

You can read about the standard implementations of the Burrito DB in this directory.

The other Markdown files here are for the specifications of the individual waiters.

The most important waiters would be:
- [`burrito_recursive`](burrito_recursive.md)
- [`burrito_asymmetric_box`](burrito_asymmetric_box.md)
- [`burrito_symmetric_box`](burrito_symmetric_box.md)