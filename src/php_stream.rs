use ext_php_rs::ffi::{php_stream, php_stream_context, zend_string};
use std::ffi::{c_char, CString};
use std::io::Read;
use std::os::raw::c_int;
use std::ptr;

unsafe extern "C" {
    fn _php_stream_open_wrapper_ex(
        path: *const ::std::os::raw::c_char,
        mode: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
        opened_path: *mut *const zend_string,
        context: *mut php_stream_context,
    ) -> *mut php_stream;

    fn _php_stream_read(
        stream: *mut php_stream,
        buf: *mut ::std::os::raw::c_char,
        count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    fn _php_stream_eof(stream: *mut php_stream) -> bool;
}

pub struct PhpStream(*mut php_stream);

impl PhpStream {
    pub fn open(path: &str, mode: &str) -> Result<Self, String> {
        let mode = CString::new(mode).expect("invalid mode");
        let path = CString::new(path).expect("invalid mode");

        let stream = unsafe {
            _php_stream_open_wrapper_ex(
                path.as_ptr().cast(),
                mode.as_ptr().cast(),
                8, // REPORT_ERRORS
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        if stream.is_null() {
            Err("Error opening file".into())
        } else {
            Ok(Self(stream))
        }
    }
}

impl Read for PhpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let buf_len = c_int::try_from(buf.len()).unwrap();
        let read = unsafe { _php_stream_read(self.0, buf.as_mut_ptr() as *mut c_char, buf_len) };
        if read == -1 {
            if unsafe { _php_stream_eof(self.0) } {
                Ok(0)
            } else {
                Err(std::io::Error::other("unable to read stream"))
            }
        } else {
            Ok(usize::try_from(read).unwrap())
        }
    }
}
