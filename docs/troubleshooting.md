# Troubleshooting

This section collects common issues encountered while building or running the extension, along with diagnostic steps.

## Build failures

### `configure: error: cargo command missing`

Install Rust and ensure `cargo` is available on the `PATH`. On Debian-based systems run `apt install rustc cargo`; on Alpine use `apk add cargo`.

### `phpize: command not found`

Install the PHP development tools for your distribution (e.g. `apt install php-dev`, `dnf install php-devel`, `apk add php8-dev`). Re-run `phpize` after installation.

### Linking errors involving OpenSSL

MRML depends on OpenSSL. Install the appropriate development package (`libssl-dev`, `openssl-devel`, `openssl-dev` depending on the distribution) before running `./configure`.

## Rendering errors

### `Mjml\Exception\RenderException: unable to open file`

The include loader in [`src/mjml.rs`](../src/mjml.rs) uses `PhpStream::open()` to resolve `<mj-include>` paths. Verify that the path exists, the PHP process can reach it, and that the stream wrapper is registered. The integration test [`tests/0001-include.phpt`](../tests/0001-include.phpt) demonstrates a successful HTTPS include.

### `Mjml\Exception\RenderException: unable to load the template file`

This message indicates the template was opened but not fully read. Inspect file permissions or network connectivity if using remote streams. The loader wraps IO failures with a descriptive cause via `IncludeLoaderError::with_cause()`.

### Unexpected HTML output

Ensure renderer options are set correctly. For example, passing a non-boolean `disable_comments` triggers a `TypeError`. Review [Configuration](./configuration.md) for valid option types.

## Runtime diagnostics

Enable verbose logging during development:

```sh
./configure --enable-cargo-debug
make clean && make
```

The debug artefact preserves symbols, making it easier to inspect crashes or submit useful reports. When reporting issues, include the MJML template and mention whether the failure stems from string or file rendering (see [`tests/0004-render-file.phpt`](../tests/0004-render-file.phpt) for reference).
