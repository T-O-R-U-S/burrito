## Standard Fields

These are fields that are reserved for applications to use, and should not be used by providers.

They are optional, and are not required to be present in every burrito entry. They are useful for categorizing entries.

### Sync entries

1. `uuid`
- A unique identifier for the entry represented as a BSON UUID. Use this to keep track of entries across multiple devices, and multiple files (BSON UUID).

2. `created`
- The time the entry was created (BSON Date).

3. `modified`
- The time the entry was last modified (BSON Date).

### Metadata

1. `title`
- A human-readable title for the entry (BSON String).

2. `description`
- A human-readable description for the entry (BSON String).

3. `notes`
- A BSON dictionary containing the note titles as keys, and the note contents (BSON String) as values.

4. `tags`
- A BSON array of arbitrary tags to categorize the entry (BSON String Array).

5. `starred`
- A boolean indicating whether the entry is starred (BSON Boolean).

6. `user_identifier`
- A string that contains a generic user identifier (email, username, phone number, etc.) (BSON String).

7. `user_name`
- A string that contains the user's name (BSON String).

8. `email`
- A string that contains the user's email address (BSON String).

9. `phone`
- A string that contains the user's phone number (BSON String).

10. `url`
- A string that contains the service provider's URL (BSON String).

11. `provider`
- A string that contains the service provider's name (BSON String).

12. `icon`
- A BSON binary containing the icon image (BSON Binary).