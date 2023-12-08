// Copyright (c) Tribufu. All Rights Reserved.

use libc::{c_char, c_int};
use std::ffi::CString;
use std::ptr;

#[repr(C)]
pub struct FHttpRequest {
    pub url: *const c_char,
    pub method: *const c_char,
    pub headers: *const FHttpHeaders,
    pub body: *const c_char,
}

#[repr(C)]
pub struct FHttpResponse {
    pub status_code: c_int,
    pub headers: *const FHttpHeaders,
    pub body: *const c_char,
}

#[repr(C)]
pub struct FHttpHeaders {
    pub entries: *const FHttpHeader,
    pub count: usize,
}

#[repr(C)]
pub struct FHttpHeader {
    pub name: *const c_char,
    pub value: *const c_char,
}

#[no_mangle]
pub extern "C" fn mintaka_http_get(url: *const c_char) -> FHttpResponse {
    let url_str = unsafe { std::ffi::CStr::from_ptr(url).to_string_lossy().into_owned() };
    let response = reqwest::blocking::get(&url_str);

    match response {
        Ok(res) => {
            let status_code = res.status().as_u16() as c_int;
            let body = res.text().unwrap_or_default();

            let c_body = CString::new(body).expect("Failed to convert body to CString");
            let body_ptr = c_body.into_raw();

            FHttpResponse {
                status_code,
                headers: ptr::null_mut(),
                body: body_ptr,
            }
        }
        Err(_) => FHttpResponse {
            status_code: 0,
            headers: ptr::null_mut(),
            body: ptr::null_mut(),
        },
    }
}

#[no_mangle]
pub extern "C" fn mintaka_http_post(url: *const c_char, body: *const c_char) -> FHttpResponse {
    let url_str = unsafe { std::ffi::CStr::from_ptr(url).to_string_lossy().into_owned() };
    let body_str = unsafe {
        std::ffi::CStr::from_ptr(body)
            .to_string_lossy()
            .into_owned()
    };

    let client = reqwest::blocking::Client::new();
    let response = client.post(&url_str).body(body_str).send();

    match response {
        Ok(res) => {
            let status_code = res.status().as_u16() as c_int;
            let body = res.text().unwrap_or_default();

            let c_body = CString::new(body).expect("Failed to convert body to CString");
            let body_ptr = c_body.into_raw();

            FHttpResponse {
                status_code,
                headers: ptr::null_mut(),
                body: body_ptr,
            }
        }
        Err(_) => FHttpResponse {
            status_code: 0,
            headers: ptr::null_mut(),
            body: ptr::null_mut(),
        },
    }
}

#[no_mangle]
pub extern "C" fn mintaka_http_free_response(response: FHttpResponse) {
    if !response.body.is_null() {
        unsafe {
            let _ = CString::from_raw(response.body as *mut c_char);
        }
    }
}
