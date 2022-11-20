use cortex_m_semihosting::hprintln;
use core::ptr::write_volatile;

/** memory mapped io: 
 * it is setting for register.
 *  compiler make memory not to optimize variable access.  
 **/
// register to control the timer. 
const CSR_ADDR: usize = 0xE000_E010;
// register to reload a value. 
const RVR_ADDR: usize = 0xE000_E014;
// register to keep a current value. 
const CVR_ADDR: usize = 0xE000_E018;

pub fn init() {
    hprintln!("Systick init").unwrap();

    // they are unsafe functions.
    unsafe {
        write_volatile(CSR_ADDR as *mut u32, 0);
        write_volatile(RVR_ADDR as *mut u32, 1 << 23);
        write_volatile(CVR_ADDR as *mut u32, 0x3);
    }
}