# `a800xl-utils`

Number of utilities for experimental `mos-a800xl-none` target, provided by https://github.com/mrk-its/rust/tree/mos_target

## API Docs
https://docs.rs/a800xl-utils

## Running examples

The easiest way is to use provided `devcontainer.json` configuration for vscode:

1. Configure Visual Studio Code with `Remote - Containers` extension
2. Open this project inside devcontainer
3. on terminal run:
    ```
    cargo build --example kbdinfo --release
    ```
4. resulting atari executable (xex) is `target/mos-a800xl-none/release/examples/kbdinfo`

## License

All source code (including code snippets) is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
