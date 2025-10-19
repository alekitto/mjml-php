# Usage

Import the classes provided by the extension and instantiate `Mjml\\Mjml` to render templates into responsive HTML emails.

```php
use Mjml\\Mjml;
use Mjml\\Exception\\RenderException;

$mjml = new Mjml([
    'disable_comments' => true,
    'social_icon_origin' => 'https://static.example.com/icons/',
]);
```

## Rendering MJML strings

Use [`Mjml::render()`](./api.md#method-render) to convert MJML markup into an [`Mjml\\Email`](./api.md#class-mjmlemail) object. The result holds the generated HTML body and metadata extracted from `<mj-head>`.

```php
$template = '<mjml><mj-body><mj-text>Hello world!</mj-text></mj-body></mjml>';
$email = $mjml->render($template);

echo $email->getBody();
```

The Rust implementation in [`src/mjml.rs`](../src/mjml.rs) parses the string with `mrml::parse_with_options()`, captures the title and preview from the head section, and returns them through the `Email` value object.

## Rendering files and stream wrappers

[`Mjml::renderFile()`](./api.md#method-renderfile) accepts any path supported by PHP stream wrappers thanks to the `PhpStreamLoader`. This enables pulling remote templates or loading from custom streams.

```php
// Local file
$newsletter = $mjml->renderFile(__DIR__ . '/templates/newsletter.mjml');

// Remote include via HTTPS (mirrors tests/0001-include.phpt)
$external = $mjml->render('<mjml><mj-body><mj-include path="https://example.com/partials/header.mjml" /></mj-body></mjml>');
```

The integration test [`tests/0004-render-file.phpt`](../tests/0004-render-file.phpt) demonstrates reading from the filesystem, while [`tests/0001-include.phpt`](../tests/0001-include.phpt) shows an external include fetched over HTTPS.

## Handling errors

Both `render()` and `renderFile()` throw `Mjml\\Exception\\RenderException` when MRML reports parsing or rendering issues, such as invalid markup or missing includes. Wrap calls in `try/catch` to recover gracefully:

```php
try {
    $mjml->render('<mjml><mj-body><mj-text>OK</mj-text></mj-body></mjml>');
} catch (RenderException $exception) {
    error_log('MJML render failed: ' . $exception->getMessage());
}
```

The Rust layer constructs the exception via [`PhpException::new`](../src/mjml.rs) and associates it with the class entry registered in [`src/exception.rs`](../src/exception.rs).

## Accessing rendered output

The returned `Mjml\\Email` object surfaces helper methods defined in [`src/mjml.rs`](../src/mjml.rs):

- `getBody(): string` – the HTML markup.
- `getTitle(): string|null` – the `<mj-title>` contents when present.
- `getPreview(): string|null` – the `<mj-preview>` contents when present.
- `__toString(): string` – convenience proxy to the body.

Use these accessors to store or transform the HTML, set an email subject line, or embed preview text in transactional mail systems.
