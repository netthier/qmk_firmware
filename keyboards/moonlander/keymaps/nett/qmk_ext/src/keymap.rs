use core::slice;
use qmk::bindgen::{raw_hid_send, rgb_matrix_mode, rgb_matrix_effects};

#[no_mangle]
pub extern "C" fn raw_hid_receive_rs(data: *mut u8, length: u8) {
    unsafe { raw_hid_send(data, length); }
    let data = unsafe { slice::from_raw_parts(data, length as usize) };
    unsafe {
        rgb_matrix_mode(rgb_matrix_effects::RGB_MATRIX_TYPING_HEATMAP as u8);
    }
}