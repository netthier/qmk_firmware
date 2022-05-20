use core::slice;
use qmk::bindgen::{raw_hid_send, rgb_matrix_mode, rgb_matrix_effects, keyrecord_t};

#[no_mangle]
pub extern "C" fn raw_hid_receive_rs(data: *const u8, length: u8) {
    unsafe { raw_hid_send(data as *mut u8, length); }
}

#[no_mangle]
pub extern "C" fn process_record_user_rs(keycode: u16, record: *const keyrecord_t) -> bool {
    true
}