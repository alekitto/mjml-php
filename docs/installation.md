# Installation

The `mjml` extension ships as source code and can be built either manually or through Composer packaging. The runtime itself only depends on PHP and the compiled Rust artefact, but building requires the MRML toolchain.

## Prerequisites

- PHP **8.0+** with development headers (`php-dev`, `php-devel`, etc.).
- [Rust](https://www.rust-lang.org/) with `cargo` available on the `PATH`.
- Build dependencies required by MRML, including a C toolchain (`build-essential`, `clang`, or similar) and SSL development libraries (`libssl-dev`, `openssl-devel`).
- `phpize` and `php-config` from your PHP installation.

> **Tip:** On Debian/Ubuntu install `php-dev rustc cargo build-essential libclang-dev libssl-dev`. On Alpine use `apk add php-dev cargo clang-dev openssl-dev build-base`.

## Build from source

Clone the repository and compile the extension:

```sh
git clone https://github.com/kcs-dev/mjml-php.git
cd mjml-php
phpize
./configure            # pass --enable-cargo-debug for debug builds
make
sudo make install      # installs mjml.so into the PHP extension directory
```

Enable the extension in `php.ini` (see [Configuration](./configuration.md)) or load it ad-hoc during development:

```sh
php -dextension=./target/release/mjml.so your-script.php
```

## Composer package (PIE)

The project is published as a Composer package (`kcs/mjml`). Use [PIE](https://github.com/php/pie) to pull the prebuilt artefact into your environment:

```sh
pie install kcs/mjml
```

PIE downloads the release, places `mjml.so` into the configured extension directory, and enables it when possible. Configuration still happens via `php.ini`.

## Verifying the installation

After enabling the module, confirm it is loaded:

```sh
php -m | grep mjml
```

Running the PHPT suite ensures the renderer works end-to-end:

```sh
php -dextension=mjml run-tests.php tests
```

Troubleshooting steps are covered in [Troubleshooting](./troubleshooting.md).
