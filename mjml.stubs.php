<?php

declare(strict_types=1);

// Stubs for mjml

namespace Mjml {
    use Mjml\Exception\RenderException;
    use Stringable;
    use TypeError;

    class Email implements Stringable {
        /**
         * Gets the email title/subject if set.
         */
        public function getTitle(): ?string {}

        /**
         * Gets the email preview text, if present.
         */
        public function getPreview(): ?string {}

        /**
         * Gets the email HTML body.
         */
        public function getBody(): string {}

        /**
         * @inheritDoc
         */
        public function __toString(): string {}
    }

    class Mjml {
        /**
         * Constructor.
         *
         * Accepts the following options:
         * - disable_comments: If true, do not include comments in the HTML output
         * - social_icon_origin: Base URL for mj-social-element images
         * /- fonts: Key-value array of fonts used in the email body
         *
         * @param array{disable_comments?: bool, social_icon_origin?: string, fonts?: array{string, string}}|null $options
         *
         * @throws TypeError if passed options are of wrong types.
         */
        public function __construct(?array $options = null) {}

        /**
         * Returns the default fonts hashmap used in rendered emails.
         *
         * @return array<string, string>
         */
        static public function defaultFonts(): array {}

        /**
         * Renders a MJML template into an email-friendly HTML markup.
         *
         * @throws RenderException
         */
        public function render(string $mjml): Email {}

        /**
         * Render a MJML file.
         * PHP Stream wrappers are supported.
         *
         * @throws RenderException
         */
        public function renderFile(string $path): Email {}
    }
}

namespace Mjml\Exception {
    use Exception;

    class RenderException extends Exception {
    }
}
