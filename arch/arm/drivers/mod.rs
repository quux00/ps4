/* driver::mod.rs */

use super::cpu::interrupt;
use super::io;
use core::option::{Option, None};
use core::vec;

pub unsafe fn init(table: interrupt::table) {
	// See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dai0235c/index.html
	table.enable(6, keypress as u32);
}

pub static mut keydown: Option<extern unsafe fn(char)> = None;
pub static mut read_char: Option<extern fn()->char> = None;

#[no_mangle]
pub unsafe fn keypress() {
	keydown.map(|f| {
		let x = *io::UART0 as u8 as char;
		f(x)
	}
	);
	asm!("pop {r11, lr}
	  subs pc, lr, #4");
}
