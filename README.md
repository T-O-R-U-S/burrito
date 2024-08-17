# Burrito -- Secrets n' Sauce

A simple, monadic, encryption-agnostic database for secrets.

## Why?

After picking up KeePassXC, I decided to have my own go of making a password database format that could rival or even
surpass KDBX4.

...and uh... I'm still working on it.

## How?

The burrito database uses BSON under-the-hood.

This is convenient because it's a binary format, and it's easy to parse. This makes it easier to store
encrypted data compared to YAML/TOML/JSON, which are all text-based formats.

## Monadic!?

Monads! Monads are awesome! They are your `Option<T>`s and `Result<T, E>`s, for the uninitiated.

A monad is like a burrito. It's a container that holds a value, and it's also a function that can be used to transform
the value inside the burrito.

The burrito database is a monad.

## Why not other formats?

Encryption produces binary data, and binary data doesn't work with 100% of human-readable serialization formats, unless
I want to use base64, which I do not because it's [wack](https://eprint.iacr.org/2022/361).

BSON has some built-in markers for encrypted and sensitive data, which is a nice plus.

You can use `serde` to export to any format you want, but I decided upon BSON because it's a binary format that is
purpose-made for this.

## Guaranteed fields:

"Guaranteed" fields are guaranteed to be present in every entry, and they are intended to be used by applications
that process the database.

- `provider`: The name of the provider that created the entry.
- `version`: The version of the provider that created the entry.
- `assumed_secure`: Whether the provider assumes the entry is secure (can I safely handle or share it without encrypting
  it?).

## API fields:

API fields are fields that are intended to be used by applications that use the database, e.g. to represent GUI.

- `title`: The title of the entry.
- `user_identifier`: Username, email, phone number, or other identifier of the entry.
- `url`: The URL of the entry.

## Security fields:

Sending `burrito`s over the network? You're gonna need these.

- `sym_signature`: Entries can be signed with a symmetric key to prevent tampering.
- `signature`: Entries can be signed with a public key to prevent tampering.
- There is a builtin provider in the library, `burrito_asymmetric_box`, for encrypted entries.

### Disclaimer:

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.