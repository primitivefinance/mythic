# bugs

- Reverts in contracts can cause update model to fail, blocking all updates from accruing.
- Defaults in the saved user data can be set and cause operations to silently fail, one example is the Contacts is a hashmap, so its default macro will just assign a blank hashmap. However, the `add` method will only set the contact if the specific address book exists, which would exist if contacts was initialized with new().
