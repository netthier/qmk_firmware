use alloc::ffi::CString;
use core::slice;
use core::str;
use num_enum::TryFromPrimitive;
use qmk::bindgen::{
    keyrecord_t, planck_ez_keycodes::ML_SAFE_RANGE, rgb_matrix_effects, rgb_matrix_mode
};

use crate::layout;

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static keymaps: layout::Keymaps = layout::KEYMAPS;

const QMK_VERSION: &str = unsafe { str::from_utf8_unchecked(qmk::bindgen::QMK_VERSION) };

#[derive(TryFromPrimitive)]
#[repr(u32)]
pub enum CustomKeycode {
    Version = ML_SAFE_RANGE,
}

#[no_mangle]
pub extern "C" fn raw_hid_receive_rs(data: *const u8, length: u8) {
    let data = unsafe { slice::from_raw_parts(data, length as usize) };
    raw_hid_send(data);
}

#[no_mangle]
pub extern "C" fn process_record_user_rs(keycode: u16, record: *const keyrecord_t) -> bool {
    let record = unsafe { *record };
    if record.event.pressed
        && let Ok(custom) = (keycode as u32).try_into() {
         match custom {
            CustomKeycode::Version => send_string(&format!("QMK {}", QMK_VERSION)),
        }
            false
    } else {
        true
    }
}

fn raw_hid_send(data: &[u8]) {
    let ptr = data as *const _ as *mut _;
    let len = data.len().try_into().unwrap();

    unsafe {
        qmk::bindgen::raw_hid_send(ptr, len);
    }
}

fn send_string(str: &str) {
    let string = CString::new(str).unwrap();
    unsafe {
        qmk::bindgen::send_string(string.as_ptr() as *const _);
    }
}
