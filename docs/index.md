# MJML PHP Extension Documentation

Welcome to the reference documentation for the `mjml` PHP extension. The extension embeds the [MRML](https://github.com/jdrouet/mrml) engine so that MJML templates can be rendered natively from PHP without invoking external CLI tools.

## Architecture overview

The extension exposes two main classes under the `Mjml` namespace:

- [`Mjml\\Mjml`](./api.md#class-mjmlmjml) orchestrates the rendering process. It wraps MRML's parser and renderer, configuring a custom include loader that understands PHP stream wrappers such as `s3://` or `php://`. The Rust implementation lives in [`src/mjml.rs`](../src/mjml.rs) and wires `ParserOptions` and `RenderOptions` from MRML to PHP.
- [`Mjml\\Email`](./api.md#class-mjmlemail) is a lightweight value object defined in [`src/mjml.rs`](../src/mjml.rs) that surfaces the rendered HTML body, optional title, and preview text.

Rendering errors propagate as [`Mjml\\Exception\\RenderException`](./api.md#namespace-mjmlexception) instances. The exception class is declared in [`mjml.stubs.php`](../mjml.stubs.php) and backed by the Rust static defined in [`src/exception.rs`](../src/exception.rs).

Template inclusion relies on a custom [`PhpStreamLoader`](../src/mjml.rs) that delegates file access to the PHP runtime via the bridge in [`src/php_stream.rs`](../src/php_stream.rs). This allows the engine to open templates using any registered PHP stream wrapper, mirroring what the tests in [`tests/0001-include.phpt`](../tests/0001-include.phpt) exercise.

## Documentation map

Use the following pages to explore specific topics:

- [Installation](./installation.md) – build the extension from source or consume the packaged release.
- [Configuration](./configuration.md) – toggle renderer options and integrate the extension in `php.ini`.
- [Usage](./usage.md) – learn how to render strings or files, include shared partials, and handle errors.
- [API reference](./api.md) – discover the available classes and methods with type information.
- [Troubleshooting](./troubleshooting.md) – resolve build issues, runtime errors, and missing includes.

## Building the documentation

The documentation is written in Markdown and served through [MkDocs](https://www.mkdocs.org/).

```sh
# Install MkDocs and the Material theme (optional) using pipx or pip.
pip install mkdocs mkdocs-material

# Build the static site into the ./site directory
mkdocs build

# Start a live-reload server on http://127.0.0.1:8000/
mkdocs serve
```

See the root [`mkdocs.yml`](../mkdocs.yml) for navigation and theme settings. The README also links back to this documentation so contributors can discover it easily.
