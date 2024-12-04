--TEST--
include external file
--FILE--
<?php
include __DIR__ . '/integration/0001-include.php';
?>
--EXPECTF--
<!doctype html><html xmlns="http://www.w3.org/1999/xhtml" %a