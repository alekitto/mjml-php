--TEST--
Mjml\Email::getTitle
--FILE--
<?php
include __DIR__ . '/integration/0006-title.php';
?>
--EXPECTF--
Title: Hello World!
<!doctype html><html xmlns="http://www.w3.org/1999/xhtml" %a
%aHello world%a