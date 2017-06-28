use std::os::raw::c_char;
use std::ffi::CString;
use std::collections::HashMap;

// Basic function which returns a string from Rust to Javascript
#[no_mangle]
pub fn get_data() -> *mut c_char {
    let mut data = HashMap::new();
    data.insert("Alice", "send");
    data.insert("Bob", "receive");
    data.insert("Carol", "intercept");

    let descriptions = data.iter()
        .map(|(p,a)| format!("{} likes to {} messages!", p, a))
        .collect::<Vec<_>>();

    CString::new(descriptions.join("\n"))
        .unwrap()
        .into_raw()
}

fn main() {
    // Deliberately blank
}