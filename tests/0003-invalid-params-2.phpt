--TEST--
invalid parameters 2
--FILE--
<?php
include __DIR__ . '/integration/0003-invalid-params-2.php';
?>
--EXPECTF--
Fatal error: Uncaught TypeError: Invalid option 'social_icon_origin': expected string, True given. %a