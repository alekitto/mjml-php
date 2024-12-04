<?php

// Stubs for php-mjml

namespace Mjml {
    class Mjml {
        public function __construct(?array $options) {}

        public function render(string $mjml): string {}
    }
}

namespace Mjml\Exception {
    class RenderException extends Exception {
    }
}
