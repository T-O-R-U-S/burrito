# Standard Fields

These are fields that are reserved for applications to use, and should not be used by providers.

## Required fields
<hr /> 

These are obligatory and must be present in every burrito entry.

### `provider`
The name of the provider.

### `version`
Semver version of the provider.
This is so that, if the provider changes, you can differentiate between different versions of the same provider.

## More fields

Check out the other files in this directory for more information.

## Avoiding name collisions
<hr />

Providers should use SCREAMING_SNAKE_CASE for their unique fields. Standard fields should use snake_case.

The field itself `provider` is a standard field, and should not be in screaming snake case.