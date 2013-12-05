/* driver::mod.rs */

use super::cpu::interrupt;
use super::io;
use core::option::{Option, None};
use core::vec;

pub unsafe fn init(table: interrupt::table) {
	// See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dai0235c/index.html
	table.enable(6, keypress as u32);
}

pub static mut keydown: Option<extern fn(char)> = None;
pub static mut read_char: Option<extern fn()->char> = None;

#[no_mangle]
pub unsafe fn keypress() {
	keydown.map(|f| {
		let x = *io::UART0 as u8;
		match x {
			13			=>	{ f('\n'); }
			127			=>	{ 
				f('');
				/*let a = 0x0C000008 as *u32;
				let b = *a;
				asm!("");
				if (b == 0){
					f('a');
				} else
				{
					f('b');
				}*/
				/* This isn't a real backspace */
				//f(' ');
				//f('');
			} // backspace =  = 8
			_			=>	{ f(x as char); }
		}
	}
	);
	asm!("pop {r11, lr}
	  subs pc, lr, #4");
}
