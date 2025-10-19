# mjml-php

[![Docs](https://img.shields.io/badge/docs-mkdocs-blue)](./docs/index.md)

PHP extension that embeds [MRML](https://github.com/jdrouet/mrml), the [MJML](https://mjml.io) rendering engine written in Rust, so that you can render MJML templates directly from PHP without shelling out to a CLI tool.

## Documentation

The full documentation lives under [`./docs/`](./docs/index.md) and is published as a MkDocs site. Start with the [architecture overview](./docs/index.md) and consult the dedicated guides for [installation](./docs/installation.md), [configuration](./docs/configuration.md), [usage](./docs/usage.md), [API details](./docs/api.md), and [troubleshooting](./docs/troubleshooting.md).

To preview the site locally install MkDocs and run:

```sh
composer run docs:serve
```

This command proxies to `mkdocs serve` using the navigation defined in [`mkdocs.yml`](./mkdocs.yml).

## Overview

- **Native MJML rendering** – compile templates into responsive HTML emails through a thin PHP wrapper around the Rust engine.
- **File and stream support** – render inline strings or files (including stream wrappers) through `render` and `renderFile`.
- **Configurable output** – control comments, custom fonts, and social icon URLs via constructor options.
- **Exceptions-first API** – failed renders surface as `Mjml\Exception\RenderException`, enabling straightforward error handling.

## Requirements

- PHP **8.0 or newer** with the development headers (`php-dev`, `php-devel`, or similar).
- [Rust](https://www.rust-lang.org/) toolchain with **Cargo** available in `PATH` (the build uses `cargo build`).
- Build dependencies required by MRML, for example `build-essential`, `gcc`, `make`, `libclang-dev`, `openssl`, `libssl-dev`, and `git`.
- `phpize` and `php-config` from your PHP installation.

> **Note:** On Alpine Linux install `php-dev`, `cargo`, `clang-dev`, `openssl-dev`, and `build-base`. On Debian/Ubuntu use `apt install php-dev rustc cargo build-essential libclang-dev libssl-dev`.

## Installation

### Build from source

```sh
phpize
./configure            # add --enable-cargo-debug to build a debug artefact
make
sudo make install      # installs mjml.so into the active PHP extension dir
```

After installation, enable the extension in your `php.ini` (see [Configuration](#configuration)). During development you can load the module from the build tree instead of running `make install`:

```sh
php -dextension=./target/release/mjml.so your-script.php
```

### Composer / packaged distribution

The project is published as a Composer package (`kcs/mjml`). You can pull the extension into your project with:

```sh
pie install kcs/mjml
```

PIE will place the extension artefact under the PHP extension directory (and enable it, if possible).

### Configuration

Add one of the following lines to your PHP configuration:

```ini
; system-wide configuration
extension=mjml

; or provide an absolute path when running from the build tree
extension=/path/to/mjml-php/target/release/mjml.so
```

When using PHP-FPM or Apache, restart the service so the new module is loaded.

## Usage

```php
<?php

use Mjml\Mjml;
use Mjml\Exception\RenderException;

$mjml = new Mjml(["disable_comments" => true]);

try {
    // Render a MJML string and fetch the body HTML.
    $email = $mjml->render('<mjml><mj-body><mj-text>Hello world!</mj-text></mj-body></mjml>');
    echo $email->getBody();

    // Render from a file or stream. Stream wrappers such as s3:// are supported.
    $newsletter = $mjml->renderFile(__DIR__ . '/templates/newsletter.mjml');
    file_put_contents(__DIR__ . '/build/newsletter.html', $newsletter->getBody());

    // Inspect the default fonts shipped with MRML.
    $fonts = Mjml::defaultFonts();
    printf("Roboto is loaded from %s\n", $fonts['Roboto']);

    // Render a template that includes reusable components.
    $withPartials = $mjml->renderFile(__DIR__ . '/templates/with-includes.mjml');
    echo $withPartials->getTitle();
} catch (RenderException $e) {
    // Handle invalid MJML gracefully (syntax error, missing include, …).
    error_log('MJML render failed: ' . $e->getMessage());
}
```

`render` and `renderFile` both return an instance of `Mjml\Email`, exposing:

- `getTitle(): string|null`
- `getPreview(): string|null`
- `getBody(): string`

## Constructor options

Pass an associative array to `new Mjml($options)` to tweak the renderer:

| Option               | Type                    | Default                                                     | Notes                                                                                                                    |
|----------------------|-------------------------|-------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------|
| `disable_comments`   | `bool`                  | `false`                                                     | Non-boolean values raise a `TypeError`. When `true`, HTML comments emitted by MRML are stripped.                         |
| `social_icon_origin` | `string`                | `https://www.mailjet.com/images/theme/v1/icons/ico-social/` | Must be a non-empty string. Controls the base URL used by `<mj-social-element>` icons.                                   |
| `fonts`              | `array<string, string>` | `Mjml::defaultFonts()`                                      | Keys are font names and values the font URL. Passing anything other than string keys/values triggers a validation error. |

### Default fonts

```php
print_r(Mjml::defaultFonts());
```

Outputs an associative array similar to:

| Font name  | URL                                                                |
|------------|--------------------------------------------------------------------|
| Open Sans  | https://fonts.googleapis.com/css?family=Open+Sans:300,400,500,700  |
| Droid Sans | https://fonts.googleapis.com/css?family=Droid+Sans:300,400,500,700 |
| Lato       | https://fonts.googleapis.com/css?family=Lato:300,400,500,700       |
| Roboto     | https://fonts.googleapis.com/css?family=Roboto:300,400,500,700     |
| Ubuntu     | https://fonts.googleapis.com/css?family=Ubuntu:300,400,500,700     |

## Troubleshooting

- **`configure: error: cargo command missing`** – install Rust and ensure `cargo` is in your `PATH`.
- **`phpize` not found** – install PHP development tools (`apt install php-dev`, `dnf install php-devel`, etc.).
- **Missing includes during rendering** – check that the `mj-include` paths exist and that PHP has permission to read them (see [`tests/0001-include.phpt`](./tests/0001-include.phpt) for an example layout).
- **Segmentation faults or crashes** – rebuild in debug mode (`./configure --enable-cargo-debug && make clean && make`) and open an issue with the failing template.

## License

Licensed under the [MIT License](./LICENSE).

## Contributing

1. Fork the repository and create a feature branch.
2. Build the extension locally (see [Build from source](#build-from-source)).
3. Run the PHPT test suite before submitting a pull request:

   ```sh
   php -dextension=./modules/mjml.so run-tests.php tests
   ```

   Integration scenarios live under [`tests/integration/`](./tests/integration/).
4. Follow the existing coding style and commit conventions, then open a pull request.
