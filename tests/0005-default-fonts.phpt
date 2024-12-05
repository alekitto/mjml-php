--TEST--
Mjml\Mjml::defaultFonts
--FILE--
<?php
include __DIR__ . '/integration/0005-default-fonts.php';
?>
--EXPECT--
array(5) {
  ["Droid Sans"]=>
  string(66) "https://fonts.googleapis.com/css?family=Droid+Sans:300,400,500,700"
  ["Lato"]=>
  string(60) "https://fonts.googleapis.com/css?family=Lato:300,400,500,700"
  ["Open Sans"]=>
  string(65) "https://fonts.googleapis.com/css?family=Open+Sans:300,400,500,700"
  ["Roboto"]=>
  string(62) "https://fonts.googleapis.com/css?family=Roboto:300,400,500,700"
  ["Ubuntu"]=>
  string(62) "https://fonts.googleapis.com/css?family=Ubuntu:300,400,500,700"
}