#![cfg_attr(windows, feature(abi_vectorcall))]

mod exception;
mod mjml;
mod php_stream;

use crate::exception::RENDER_EXCEPTION;
use ext_php_rs::builders::ClassBuilder;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ce;

use mjml::Mjml;

#[php_startup]
pub fn startup() {
    let ce = ClassBuilder::new("Mjml\\Exception\\RenderException")
        .extends(ce::exception())
        .build()
        .expect("Failed to build RenderException");

    #[allow(static_mut_refs)]
    unsafe {
        RENDER_EXCEPTION.replace(ce)
    };
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
