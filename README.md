# mjml-php

PHP wrapper for [MRML](https://github.com/jdrouet/mrml), the [MJML](https://mjml.io) implementation in Rust.
Rust needs to be available to build this extension from source.

## Building, extending PHP

```sh
$ phpize
$ ./configure
$ make
$ make install
# in php.ini set extension=mjml.so or:
$ php -dextension=path-to-project/target/release/mjml.so <your php file>
```

# PHP

```php
<?php

$mjml = new Mjml\Mjml();
$rendered = $mjml->render('<mjml><mj-body><mj-text>Hello world!</mj-text></mj-body></mjml>');

echo $rendered; // Email-friendly HTML
```

## Classes and Methods

The extension exposes a `Mjml\Mjml` class to parse and render mjml into HTML. You can use the following methods:

- `public function render(string $mjml): Mjml\Email`: render a mjml string into HTML
- `public function renderFile(string $path): Mjml\Email`: read the specified file and render the mjml content into HTML (stream wrappers are supported).
- `public static function defaultFonts(): array`: returns a hashmap with the default fonts

`render` and `renderFile` methods return a `Mjml\Email` object which exposes the following methods:

- `public function getTitle(): string|null`: returns the content of the `<mj-title>` tag, if set
- `public function getPreview(): string|null`: returns the content of the `<mj-preview>` tag, if present
- `public function getBody(): string`: returns the HTML email body

## Options

While constructing the Mjml object, you can pass an array with the following options:

| Option             | Description                                         | Default                                                   |
|--------------------|-----------------------------------------------------|-----------------------------------------------------------|
| disable_comments   | If true, do not include comments in the HTML output | false                                                     |
| social_icon_origin | Base URL for mj-social-element images               | https://www.mailjet.com/images/theme/v1/icons/ico-social/ |
| fonts              | Key-value array of fonts used in the email body     | See [default fonts](#default-fonts)                       |

### Default fonts

By default, this library uses the following fonts:

| Font name  | URL                                                                |
|------------|--------------------------------------------------------------------|
| Open Sans  | https://fonts.googleapis.com/css?family=Open+Sans:300,400,500,700  |
| Droid Sans | https://fonts.googleapis.com/css?family=Droid+Sans:300,400,500,700 |
| Lato       | https://fonts.googleapis.com/css?family=Lato:300,400,500,700       |
| Roboto     | https://fonts.googleapis.com/css?family=Roboto:300,400,500,700     | 
| Ubuntu     | https://fonts.googleapis.com/css?family=Ubuntu:300,400,500,700     |
