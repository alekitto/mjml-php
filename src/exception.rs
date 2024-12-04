use ext_php_rs::zend::ClassEntry;

pub static mut RENDER_EXCEPTION: Option<&'static ClassEntry> = None;
