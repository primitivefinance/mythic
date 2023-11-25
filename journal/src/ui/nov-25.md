# bugs

Found a bug where if there are duplicate entries in the address book (i.e., same key with different labels), it will break the execution flow when trying to find the artifact, because it uses the label as the path to find the contract's artifact.