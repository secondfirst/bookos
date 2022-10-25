#![no_main]
#![no_std]

use core::panic::PanicInfo;

// The reset vector, a pointer into the reset handler.
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() ->  ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    let _x = 42;

    // 無限にループして値を返さない。
    loop {}
}