<?php

use Mjml\Exception\RenderException;

$mjml = new \Mjml\Mjml();

try {
    $mjml->render('<mjml><mj-body><mj-unknown');
    assert(false, 'render() should throw on invalid markup');
} catch (RenderException $e) {
    assert($e instanceof \Exception);
}

try {
    // @: the stream wrapper also warns about the missing file.
    @$mjml->renderFile(__DIR__ . '/../data/does-not-exist.mjml');
    assert(false, 'renderFile() should throw on a missing file');
} catch (RenderException $e) {
    assert($e instanceof \Exception);
}

echo 'OK';
