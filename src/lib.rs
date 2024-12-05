#![cfg_attr(windows, feature(abi_vectorcall))]

mod exception;
mod mjml;
mod php_stream;

use crate::exception::RENDER_EXCEPTION;
use ext_php_rs::builders::ClassBuilder;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ce;

use mjml::Email;
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

#[cfg(test)]
mod integration {
    use std::env;

    use std::process::Command;
    use std::sync::Once;

    static BUILD: Once = Once::new();

    fn setup() {
        BUILD.call_once(|| {
            assert!(Command::new("cargo")
                .arg("build")
                .output()
                .expect("failed to build extension")
                .status
                .success());
        });
    }

    pub fn run_php(file: &str) -> bool {
        setup();
        let mut path = env::current_dir().expect("Could not get cwd");
        path.push("target");
        path.push("debug");
        path.push(if std::env::consts::DLL_EXTENSION == "dll" {
            "mjml"
        } else {
            "libmjml"
        });
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
