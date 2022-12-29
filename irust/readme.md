
# rust fun

inspired by:

- tiktoken https://github.com/openai/tiktoken
- https://maxwellrules.com/programming/rusty-python.html

want to learn some rust

working through https://google.github.io/comprehensive-rust/welcome.html

## random notes

- `rustc` compiler
- `cargo` dependency manager
- `rustup` toolchain installer/updater

Static memory management at compile time:

- No uninitialized variables.
- Mostly no memory leaks1
- No double-frees.
- No use-after-free.
- No NULL pointers.
- No forgotten locked mutexes.
- No data races between threads.
- No iterator invalidation.

No undefined behavior at runtime:

- Array access is bounds checked.
- Integer overflow is defined.

Modern features: Rust is built with all the experience gained in the last 40 years.

- Enums and pattern matching.
- Generics.
- No overhead FFI.
- Great compiler errors.
- Built-in dependency manager.
- Built-in support for testing.

