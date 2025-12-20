#![cfg_attr(windows, feature(abi_vectorcall))]

mod exception;
mod mjml;
mod php_stream;

use ext_php_rs::prelude::*;
use ext_php_rs::zend::ce;
use crate::mjml::{Email, Mjml};

#[php_class]
#[php(extends(ce = ce::exception, stub = "Exception"))]
struct RenderException {}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<Mjml>()
        .class::<Email>()
        .class::<RenderException>()
}

#[cfg(test)]
mod integration {
    use std::env;
    
    use std::process::Command;
    use std::sync::Once;

    static BUILD: Once = Once::new();

    fn setup() {
        BUILD.call_once(|| {
            assert!(
                Command::new("cargo")
                    .arg("build")
                    .output()
                    .expect("failed to build extension")
                    .status
                    .success()
            );
        });
    }

    pub fn run_php(file: &str) -> bool {
        setup();
        let mut path = env::current_dir().expect("Could not get cwd");
        path.push("target");
        path.push("debug");
        path.push(format!("{}mjml", std::env::consts::DLL_PREFIX));
        path.set_extension(std::env::consts::DLL_EXTENSION);
        let output = Command::new("php")
            .arg(format!("-dextension={}", path.to_str().unwrap()))
            .arg("-dassert.active=1")
            .arg("-dassert.exception=1")
            .arg("-dzend.assertions=1")
            .arg(format!("tests/integration/{}", file))
            .output()
            .expect("failed to run php file");
        if output.status.success() {
            true
        } else {
            panic!(
                "
                status: {}
                stdout: {}
                stderr: {}
                ",
                output.status,
                String::from_utf8(output.stdout).unwrap(),
                String::from_utf8(output.stderr).unwrap()
            );
        }
    }

    #[test]
    pub fn test_include_external() {
        assert!(run_php("0001-include.php"));
    }

    #[test]
    pub fn test_render_file() {
        assert!(run_php("0004-render-file.php"));
    }

    #[test]
    pub fn test_default_fonts() {
        assert!(run_php("0005-default-fonts.php"));
    }
}
