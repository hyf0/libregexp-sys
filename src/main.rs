use std::ffi::{c_char, c_int, c_uchar};

use libregexp_sys::{lre_compile, lre_exec};


const ERROR_MSG_LEN: usize = 64;



fn main() {
    let args = std::env::args().into_iter().collect::<Vec<_>>();
    println!("args: {:?}", args);

    if args.len() < 3 {
        println!("usage: {} regexp input", args[0]);
        return;
    }

    let _len: c_int;
    let ret: c_int;
    let _i: c_int;
    let bc: *const c_uchar;
    let mut error_msg = [0 as c_char; ERROR_MSG_LEN];
    let mut capture = [0 as c_uchar; 410];
    let input: *const c_char;
    let input_len;
    let capture_count: c_int;

    unsafe {
        let pat = args[1].as_ptr() as *const c_char;
        let len = args[1].len();
        bc = lre_compile(
            &mut (len as c_int) as *mut c_int,
            error_msg.as_mut_ptr(),
            64,
            pat,
            len,
            0,
            std::ptr::null_mut(),
        );

        if bc == std::ptr::null() {
            eprintln!(
                "error: {}\n",
                std::ffi::CStr::from_ptr(error_msg.as_ptr())
                    .to_str()
                    .unwrap()
            );
            return;
        }

        input = args[2].as_ptr() as *const c_char;
        input_len = args[2].len() as c_int;

        ret = lre_exec(
            capture.as_mut_ptr() as *mut *mut u8,
            bc,
            input as *const u8,
            0,
            input_len,
            0,
            std::ptr::null_mut(),
        );
        println!("ret={}\n", ret);
        if ret == 1 {
            capture_count = *capture.as_ptr() as c_int;
            println!("capture_count={}\n", capture_count);
            for i in 0..(capture_count * 2) {
                let ptr: *const c_uchar;
                ptr = capture.as_ptr().offset(i as isize);
                println!("{}:", i);
                if !ptr.is_null() {
                    print!("<nil>")
                } else {
                    print!(
                        "{}",
                        (ptr as *const u8).offset_from(input as *const u8) as u32
                    );
                }
                print!("/n")
            }
        }
    }
}
