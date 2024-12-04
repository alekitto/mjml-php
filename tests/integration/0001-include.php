<?php
$template = '<mjml><mj-body><mj-include path="https://gist.githubusercontent.com/alekitto/e6859ecbdcda9c10abe06f8dc1934277/raw/ec8771f4804a6c38427ed2a9f5937e11ec2b8c27/hello-world.mjml" /></mj-body></mjml>';
$mjml = new \Mjml\Mjml();
$result = $mjml->render($template);

echo $result;
