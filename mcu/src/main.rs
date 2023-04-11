#![no_main]
#![cfg_attr(not(test), no_std)]
// for: debug
#![allow(dead_code)]

#[no_mangle]
extern "C" fn app_run() -> ! {
    loop { }
}

#[no_mangle]
extern "C" fn __libc_init_array() {

}

// target是thumbv7em-none-eabihf时候启用
#[cfg(target_arch = "arm")]
#[panic_handler]
fn panic_abort(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}
