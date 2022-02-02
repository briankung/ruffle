use rb_sys::{
    rb_define_module, rb_define_module_function, rb_string_value_cstr, rb_utf8_str_new, VALUE,
};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_long};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[inline]
unsafe fn cstr_to_string(str: *const c_char) -> String {
    CStr::from_ptr(str).to_string_lossy().into_owned()
}

#[no_mangle]
unsafe extern "C" fn pub_shuffle(_klass: VALUE, mut input: VALUE) -> VALUE {
    let mut chars: Vec<char> = cstr_to_string(rb_string_value_cstr(&mut input))
        .chars()
        .collect();
    chars.shuffle(&mut thread_rng());

    let shuffled: String = chars.iter().collect();
    let size = shuffled.len() as c_long;

    let shuffled_cstring = CString::new(shuffled).unwrap();

    rb_utf8_str_new(shuffled_cstring.as_ptr(), size)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_ruffle() {
    let name = CString::new("Ruffle").unwrap();
    let shuffle = CString::new("shuffle").unwrap();
    let callback = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(VALUE, VALUE) -> VALUE,
            unsafe extern "C" fn() -> VALUE,
        >(pub_shuffle)
    };
    let klass = unsafe { rb_define_module(name.as_ptr()) };

    unsafe { rb_define_module_function(klass, shuffle.as_ptr(), Some(callback), 1) }
}
