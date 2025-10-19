# Configuration

Once the extension is installed, configure it and its rendering behaviour through `php.ini` and constructor options exposed by `Mjml\\Mjml`.

## Enabling the extension

Add one of the following directives to your PHP configuration files:

```ini
; System-wide configuration
extension=mjml

; Load from an absolute path when testing from the build tree
extension=/path/to/mjml-php/target/release/mjml.so
```

Restart PHP-FPM, Apache, or any long-running PHP process to pick up the new module.

## Renderer options

Pass an associative array to the `Mjml\\Mjml` constructor to tweak the MRML renderer. Input validation mirrors the checks in [`src/mjml.rs`](../src/mjml.rs) and [`mjml.stubs.php`](../mjml.stubs.php); incorrect types raise `TypeError` before rendering starts.

| Option               | Type                    | Default                                                     | Description                                                                                                                                       |
|----------------------|-------------------------|-------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
| `disable_comments`   | `bool`                  | `false`                                                     | Strip HTML comments from the generated markup. Any non-boolean values trigger a `TypeError` via [`ce::type_error()`](../src/mjml.rs).             |
| `social_icon_origin` | `string`                | `https://www.mailjet.com/images/theme/v1/icons/ico-social/` | Override the base URL MRML uses for `<mj-social-element>` icons. Empty or non-string values raise a `TypeError`.                                  |
| `fonts`              | `array<string, string>` | `Mjml::defaultFonts()`                                      | Supply custom font URLs. Keys become font names and values the font source. Non-string entries lead to validation errors in the constructor loop. |

### Retrieving default fonts

Call the static `Mjml::defaultFonts()` helper to inspect or override the built-in font map provided by MRML:

```php
$fonts = Mjml::defaultFonts();
printf("Roboto loads from %s\n", $fonts['Roboto']);
```

The method iterates over `mrml::render::default_fonts()` and exposes the mapping as a PHP associative array (see [`src/mjml.rs`](../src/mjml.rs)).

## Stream wrappers and includes

When templates use `<mj-include>`, MRML relies on a custom include loader implemented in [`src/mjml.rs`](../src/mjml.rs). It delegates IO to `PhpStream::open()` from [`src/php_stream.rs`](../src/php_stream.rs), allowing any registered PHP stream wrapper—such as `https://`, `s3://`, or `file://`—to participate. See the usage examples in [`tests/0001-include.phpt`](../tests/0001-include.phpt) for an external include fetched over HTTPS.

If an include cannot be opened or read, MRML raises an error that surfaces as `Mjml\Exception\RenderException`. Handling these exceptions is covered in [Usage](./usage.md#handling-errors) and [Troubleshooting](./troubleshooting.md#rendering-errors).
