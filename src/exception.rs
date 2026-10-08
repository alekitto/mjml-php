use ext_php_rs::class::RegisteredClass;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::{ClassEntry, ce};

#[php_class]
#[php(name = "Mjml\\Exception\\RenderException")]
#[php(extends(ce = ce::exception, stub = "Exception"))]
pub struct RenderException {}

/// The class entry registered by the latest MINIT, read at throw time so it stays valid when the SAPI
/// restarts the module (FrankenPHP reboots all PHP threads on workers/restart and opcache_reset()).
pub fn render_exception() -> &'static ClassEntry {
    RenderException::get_metadata().ce()
}
