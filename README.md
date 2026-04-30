# zizmor-zed

A Zed extension for [zizmor]. Available on the [Zed marketplace].

[zizmor]: https://docs.zizmor.sh
[Zed marketplace]: https://zed.dev/extensions/zizmor

This extension exposes zizmor's language server to Zed, providing
in-editor hints for zizmor's findings.

> [!IMPORTANT]
> This extension is in an early stage of development; you _will_ encounter bugs
> and missing features. Please report them!

## Installation

> [!IMPORTANT]
> You **must** have `zizmor` v1.11.0 or later installed; earlier versions
> do not include LSP support.
>
> If `zizmor --version` shows a version below 1.11.0, you **must** update
> `zizmor` to use this extension.

To use this extension, you must have `zizmor` installed on your system.
See [zizmor's installation documentation] for system-appropriate instructions.

[zizmor's installation documentation]: https://docs.zizmor.sh/installation/

Once you have `zizmor` installed, you can install this extension from
Zed's [extensions marketplace].

[extensions marketplace]: https://zed.dev/extensions

## Development

To build the extension locally, make sure you have the `wasm-32-wasip2` target installed:

```bash
rustup target add wasm32-wasip2
```

Then, you can build the extension with:

```bash
cargo build --release --target wasm32-wasip2
```

To load a development build of the extension into Zed, open
the "Extensions" tab via the command palette,
click "Install Dev Extension," and select the directory
you've cloned this repository into.
