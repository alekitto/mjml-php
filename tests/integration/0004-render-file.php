<?php
$mjml = new \Mjml\Mjml();
$result = $mjml->renderFile(__DIR__ . '/../data/t01.mjml');

echo $result;
