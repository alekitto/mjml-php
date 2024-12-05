<?php
$mjml = new \Mjml\Mjml();
$email = $mjml->renderFile(__DIR__ . '/../data/t06.mjml');

echo "Title: " . $email->getTitle() . "\n";
echo $email;
