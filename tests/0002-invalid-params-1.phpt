--TEST--
invalid parameters 1
--FILE--
<?php
include __DIR__ . '/integration/0002-invalid-params-1.php';
?>
--EXPECTF--
Fatal error: Uncaught Exception: Invalid option 'disable_comments': expected bool, String given. %a