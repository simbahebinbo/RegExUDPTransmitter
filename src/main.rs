extern crate libc;
use std::ffi::CString;
use std::ptr;

use libc::{c_char, c_int, c_void};

#[link(name = "pcre2-8")]
extern "C" {
    fn pcre2_compile_8(pattern: *const c_char, length: usize, options: u32, errorcode: *mut c_int, erroroffset: *mut usize, context: *mut c_void) -> *mut c_void;
    fn pcre2_match_data_create_from_pattern_8(pattern: *const c_void, context: *mut c_void) -> *mut c_void;
    fn pcre2_match_8(pattern: *const c_void, subject: *const c_char, length: usize, startoffset: usize, options: u32, match_data: *mut c_void, context: *mut c_void) -> c_int;
    fn pcre2_get_ovector_pointer_8(match_data: *const c_void) -> *const usize;
    fn pcre2_match_data_free_8(match_data: *mut c_void);
    fn pcre2_code_free_8(code: *mut c_void);
}

fn main() {
    //     	\d{4}: 匹配4个数字
    //     	([^\d\s]{3,11}): 匹配3到11个不包含数字和空白字符的字符串，并捕获为结果字符串
    //     	(?=\S): 确保结果字符串右侧相邻的字符串不为空
    let pattern = CString::new(r"\d{4}([^\d\s]{3,11})(?=\S)").unwrap();
    let subject = CString::new("a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999").unwrap();

    unsafe {
        let mut errorcode: c_int = 0;
        let mut erroroffset: usize = 0;
        let code = pcre2_compile_8(pattern.as_ptr(), pattern.as_bytes().len(), 0, &mut errorcode, &mut erroroffset, ptr::null_mut());
        if code.is_null() {
            eprintln!("Failed to compile pattern");
            return;
        }

        let match_data = pcre2_match_data_create_from_pattern_8(code, ptr::null_mut());
        if match_data.is_null() {
            eprintln!("Failed to create match data");
            pcre2_code_free_8(code);
            return;
        }

        let rc = pcre2_match_8(code, subject.as_ptr(), subject.as_bytes().len(), 0, 0, match_data, ptr::null_mut());
        if rc < 0 {
            eprintln!("Failed to match pattern");
            pcre2_match_data_free_8(match_data);
            pcre2_code_free_8(code);
            return;
        }

        let ovector = pcre2_get_ovector_pointer_8(match_data);
        if ovector.is_null() {
            eprintln!("Failed to get ovector");
            pcre2_match_data_free_8(match_data);
            pcre2_code_free_8(code);
            return;
        }

        let start = *ovector.offset(2) as usize;
        let end = *ovector.offset(3) as usize;
        let result = &subject.to_bytes()[start..end];
        println!("Matched: {:?}", String::from_utf8_lossy(result));

        // 通过UDP发送结果给bash脚本
        use std::net::UdpSocket;
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to address");
        socket.send_to(result, "127.0.0.1:12345").expect("Couldn't send data");

        pcre2_match_data_free_8(match_data);
        pcre2_code_free_8(code);
    }
}
