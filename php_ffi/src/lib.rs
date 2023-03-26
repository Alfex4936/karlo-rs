use karlo_rs::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern "C" fn generate_image_ffi(
    prompt: *const c_char,
    output_prefix: *const c_char,
    api_key: *const c_char,
    batch_size: c_int,
) -> c_int {
    let prompt = unsafe { CStr::from_ptr(prompt).to_str().unwrap() };
    let output_prefix = unsafe { CStr::from_ptr(output_prefix).to_str().unwrap() };
    let api_key = unsafe { CStr::from_ptr(api_key).to_str().unwrap() };
    let batch_size = if batch_size > 0 {
        Some(batch_size as usize)
    } else {
        None
    };

    match generate_image_sync(prompt, output_prefix, api_key, batch_size) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[no_mangle]
pub extern "C" fn generate_variations_ffi(
    input_path: *const c_char,
    output_prefix: *const c_char,
    api_key: *const c_char,
    batch_size: c_int,
) -> c_int {
    let input_path = unsafe { CStr::from_ptr(input_path).to_str().unwrap() };
    let output_prefix = unsafe { CStr::from_ptr(output_prefix).to_str().unwrap() };
    let api_key = unsafe { CStr::from_ptr(api_key).to_str().unwrap() };
    let batch_size = if batch_size > 0 {
        Some(batch_size as usize)
    } else {
        None
    };

    match generate_variations_sync(input_path, output_prefix, api_key, batch_size) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}
