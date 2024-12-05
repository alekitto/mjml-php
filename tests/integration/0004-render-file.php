<?php
$mjml = new \Mjml\Mjml();
$result = $mjml->renderFile(__DIR__ . '/../data/t04.mjml');

echo $result;
