/* io::mod.rs */

use core::option::Some;
use super::drivers;

/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/BBABEGGE.html */
pub static UART0: *mut u32 = 0x101f1000 as *mut u32;

pub static UART0_IMSC: *mut u32 = (0x101f1000 + 0x038) as *mut u32;
pub static VIC_INTENABLE: *mut u32 = (0x10140000 + 0x010) as *mut u32;

pub unsafe fn write_char(c: char, address: *mut u32) {
	*address = c as u32;
}
