# wit-resource-example

This is an attempt at having one component instantiate a resource and pass it to another component.

1. The `host` calls `init` on the `extension` component.
2. The `extension` component instantiates a `builder` resource and passes it to a `register-provider` import.
3. The `facade` component receives the `builder` and immediately calls it.
4. Alternatively, the `facade` can store a reference the `builder` (not implemented).

## Usage

Simply run `make` to build both components and run the host program.
