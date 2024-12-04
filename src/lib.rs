#![cfg_attr(windows, feature(abi_vectorcall))]

use std::borrow::Cow;
use std::collections::HashMap;
use ext_php_rs::prelude::*;
use ext_php_rs::{exception::PhpException, zend::ce};
use ext_php_rs::builders::ClassBuilder;
use ext_php_rs::types::Zval;
use ext_php_rs::zend::ClassEntry;
use mrml::prelude::render::{default_fonts, RenderOptions};

static mut RENDER_EXCEPTION: Option<&'static ClassEntry> = None;

#[php_class(name = "Mjml\\Mjml")]
pub struct Mjml {
    render_options: RenderOptions
}

#[php_impl]
impl Mjml {
    pub fn __construct(options: Option<HashMap<String, &Zval>>) -> PhpResult<Self> {
        let mut opts = RenderOptions::default();

        if let Some(options) = options {
            opts.disable_comments = (|| -> PhpResult<bool> {
                let Some(zv) = options.get("disable_comments") else { return Ok(false) };
                let Some(value) = zv.bool() else {
                    return Err(PhpException::default(format!("Invalid option 'disable_comments': expected bool, {} given.", zv.get_type().to_string())));
                };

                Ok(value)
            })()?;

            opts.social_icon_origin = (|| -> PhpResult<Option<Cow<'_, str>>> {
                let Some(zv) = options.get("social_icon_origin") else { return Ok(None) };
                let Some(value) = zv.string() else {
                    return Err(PhpException::default(format!("Invalid option 'social_icon_origin': expected string, {} given.", zv.get_type().to_string())));
                };

                Ok(Some(value.into()))
            })()?;

            opts.fonts = (|| -> PhpResult<HashMap<String, Cow<'_, str>>> {
                let Some(zv) = options.get("fonts") else { return Ok(default_fonts()) };
                let Some(value) = zv.array() else {
                    return Err(PhpException::default(format!("Invalid option 'fonts': expected array, {} given.", zv.get_type().to_string())));
                };

                let mut map = HashMap::default();
                for (k, v) in value.iter() {
                    let key = k.to_string();
                    let Some(v) = v.string() else {
                        return Err(PhpException::default(format!("Invalid option 'fonts': expected string, {} given at index {}.", zv.get_type().to_string(), key)));
                    };

                    map.insert(key, v.into());
                }

                Ok(map)
            })()?;
        }

        Ok(Self {
            render_options: opts
        })
    }

    pub fn render(&self, mjml: String) -> PhpResult<String> {
        let mjml = match mrml::parse(mjml) {
            Ok(parsed) => parsed,
            Err(e) => {
                return Err(PhpException::new(e.to_string(), 0, unsafe {
                    RENDER_EXCEPTION.expect("did not set exception ce")
                }));
            }
        };

        match mjml.render(&self.render_options) {
            Ok(rendered) => Ok(rendered),
            Err(e) => Err(PhpException::new(e.to_string(), 0, unsafe {
                RENDER_EXCEPTION.expect("did not set exception ce")
            })),
        }
    }
}

#[php_startup]
pub fn startup() {
    let ce = ClassBuilder::new("Mjml\\Exception\\RenderException")
        .extends(ce::exception())
        .build()
        .expect("Failed to build RenderException");

    #[allow(static_mut_refs)]
    unsafe { RENDER_EXCEPTION.replace(ce) };
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}