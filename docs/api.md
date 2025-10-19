# API Reference

This page documents the public PHP surface exposed by the extension. Stubs live in [`mjml.stubs.php`](../mjml.stubs.php) and mirror the Rust implementation in [`src/mjml.rs`](../src/mjml.rs).

## Namespace `Mjml`

### Class `Mjml\\Email`

A value object that represents the result of a render call.

| Method                       | Description                                                                            |
|------------------------------|----------------------------------------------------------------------------------------|
| `getTitle(): string\|null`   | Returns the `<mj-title>` contents if provided. Backed by `Email::get_title()` in Rust. |
| `getPreview(): string\|null` | Returns the `<mj-preview>` contents if present.                                        |
| `getBody(): string`          | Returns the rendered HTML output.                                                      |
| `__toString(): string`       | Returns the HTML body, enabling direct `echo $email;` usage.                           |

Instances are created internally by [`Mjml::render()`](#method-render) and [`Mjml::renderFile()`](#method-renderfile) after MRML renders the template.

### Class `Mjml\\Mjml`

The main entry point for rendering MJML templates.

#### Constructor

```php
public function __construct(?array $options = null)
```

The constructor accepts an associative array with the following keys:

- `disable_comments`: `bool` – when `true`, comments are removed from the resulting HTML.
- `social_icon_origin`: `string` – base URL used for `<mj-social-element>` icons.
- `fonts`: `array<string, string>` – custom font URLs overriding MRML defaults.

Each option is validated in Rust using type guards (`zval.bool()`, `zval.string()`, etc.). Invalid types raise `TypeError` before any rendering occurs.

#### Static method `defaultFonts`

```php
public static function defaultFonts(): array<string, string>
```

Returns the MRML default font mapping. Internally, [`Mjml::default_fonts()`](../src/mjml.rs) iterates over `mrml::render::default_fonts()` and converts the result into a PHP hash table.

#### Method `render`

```php
public function render(string $mjml): Mjml\\Email
```

Parses and renders MJML markup from a string. The engine uses `mrml::parse_with_options()` with a `ParserOptions` configured to load includes through the PHP stream bridge. Rendering errors throw [`Mjml\\Exception\\RenderException`](#namespace-mjmlexception).

#### Method `renderFile`

```php
public function renderFile(string $path): Mjml\\Email
```

Opens a MJML template from disk or any PHP stream wrapper via [`PhpStream::open()`](../src/php_stream.rs). The file contents are forwarded to `render()`.

## Namespace `Mjml\\Exception`

### Class `RenderException`

Custom exception raised when MRML encounters parsing or rendering errors. The static class entry is stored in [`src/exception.rs`](../src/exception.rs) so the Rust layer can instantiate the PHP exception with `PhpException::new()`.
