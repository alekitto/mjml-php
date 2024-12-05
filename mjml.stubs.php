<?php

// Stubs for php-mjml

namespace Mjml {
    class Mjml {
        /**
         * Constructor.
         *
         * @param array{disable_comments?: bool, social_icon_origin?: string, fonts?: array{string, string}}|null $options
         */
        public function __construct(?array $options = null) {}

        /** @return array<string, string> */
        static public function defaultFonts(): array {}

        public function render(string $mjml): string {}

        public function renderFile(string $path): string {}
    }
}

namespace Mjml\Exception {
    class RenderException extends Exception {
    }
}
