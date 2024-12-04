use crate::exception::RENDER_EXCEPTION;
use crate::php_stream::PhpStream;
use ext_php_rs::exception::{PhpException, PhpResult};
use ext_php_rs::{php_class, php_impl};
use mrml::prelude::parser::loader::{IncludeLoader, IncludeLoaderError};
use mrml::prelude::parser::ParserOptions;
use mrml::prelude::render::{default_fonts, RenderOptions};
use std::borrow::Cow;
use std::io::{ErrorKind, Read};
use std::sync::Arc;

#[php_class(name = "Mjml\\Mjml")]
pub struct Mjml {
    parser_options: ParserOptions,
    render_options: RenderOptions,
}

#[php_impl]
impl Mjml {
    pub fn __construct(
        options: Option<std::collections::HashMap<String, &ext_php_rs::types::Zval>>,
    ) -> PhpResult<Self> {
        let parser_options = ParserOptions {
            include_loader: Box::new(PhpStreamLoader {}),
        };

        let mut opts = RenderOptions::default();

        if let Some(options) = options {
            opts.disable_comments = (|| -> PhpResult<bool> {
                let Some(zv) = options.get("disable_comments") else {
                    return Ok(false);
                };
                let Some(value) = zv.bool() else {
                    return Err(PhpException::default(format!(
                        "Invalid option 'disable_comments': expected bool, {} given.",
                        zv.get_type()
                    )));
                };

                Ok(value)
            })()?;

            opts.social_icon_origin = (|| -> PhpResult<Option<Cow<'_, str>>> {
                let Some(zv) = options.get("social_icon_origin") else {
                    return Ok(None);
                };
                let Some(value) = zv.string() else {
                    return Err(PhpException::default(format!(
                        "Invalid option 'social_icon_origin': expected string, {} given.",
                        zv.get_type()
                    )));
                };

                Ok(Some(value.into()))
            })()?;

            opts.fonts = (|| -> PhpResult<std::collections::HashMap<String, Cow<'_, str>>> {
                let Some(zv) = options.get("fonts") else {
                    return Ok(default_fonts());
                };
                let Some(value) = zv.array() else {
                    return Err(PhpException::default(format!(
                        "Invalid option 'fonts': expected array, {} given.",
                        zv.get_type()
                    )));
                };

                let mut map = std::collections::HashMap::default();
                for (k, v) in value.iter() {
                    let key = k.to_string();
                    let Some(v) = v.string() else {
                        return Err(PhpException::default(format!(
                            "Invalid option 'fonts': expected string, {} given at index {}.",
                            zv.get_type(),
                            key
                        )));
                    };

                    map.insert(key, v.into());
                }

                Ok(map)
            })()?;
        }

        Ok(Self {
            parser_options,
            render_options: opts,
        })
    }

    pub fn render(&self, mjml: String) -> PhpResult<String> {
        let mjml = match mrml::parse_with_options(mjml, &self.parser_options) {
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

#[derive(Debug)]
struct PhpStreamLoader {}

impl IncludeLoader for PhpStreamLoader {
    fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        let mut stream = PhpStream::open(path, "rb").map_err(|_| {
            IncludeLoaderError::new(path, ErrorKind::InvalidInput)
                .with_message("unable to open file")
        })?;

        let mut string = String::new();
        stream.read_to_string(&mut string).map_err(|err| {
            IncludeLoaderError::new(path, ErrorKind::InvalidData)
                .with_message("unable to load the template file")
                .with_cause(Arc::new(err))
        })?;

        Ok(string)
    }
}