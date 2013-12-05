/* kernel::sgash.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::vec::Vec;
use super::super::platform::*;

fn putchar(key: char) {
    unsafe {
	/*
	 * We need to include a blank asm call to prevent rustc
	 * from optimizing this part out
	 */
	asm!("");
	io::write_char(key, io::UART0);
    }
}

#[lang = "exchange_free"]
fn putstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
	putchar(*c as char);
    }
}

pub fn parsekey(x: char) {
    let x = x as u8;
    // Set this to false to learn the keycodes of various keys!
    // Key codes are printed backwards before life is hard
    if (true) {
	match x {
	    13			=>	{ prompt(); }
	    127			=>	{ putchar(''); }
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
				// backspace =  = 8
	    _			=>	{ putchar(x as char); }
	}
    }
    else {
	keycode(x);
    }
}

fn keycode(x: u8) {
    let mut x = x;
    while ( x != 0 ) {
	putchar((x%10+ ('0' as u8) ) as char);
	x = x/10;
    }
    putchar(' ');
}

pub unsafe fn init() {
    prompt();
}

fn prompt() {
    putstr(&"\nsgash > ");
}
