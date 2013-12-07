/* kernel::sgash.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::vec::Vec;
use super::super::platform::*;

pub static mut p_cstr: Option<*mut u8> = None;
pub static mut p_cstr_i: uint = 0;

pub fn putchar(key: char) {
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
pub unsafe fn output()
{
    p_cstr.map(|p| {
	let mut x = 0;
	while *(((p as uint)+x) as *char) != '\0'
	{
	    putchar(*(((p as uint)+x) as *char));
	    x += 1;
	}
    });
}
pub unsafe fn parsekey(x: char) {
    let x = x as u8;
    // Set this to false to learn the keycodes of various keys!
    // Key codes are printed backwards because life is hard
    if (true) {
	match x {
	    13			=>	{ 
		prompt(); 
		//output(); 
		p_cstr_i = 0; p_cstr.map(|p| {
		    *(p as *mut char) = '\0';
		});
	    }
	    127			=>	{ 
		putchar(''); 
		p_cstr.map(|p| {
		    p_cstr_i -= 1;
		    *(((p as uint)+p_cstr_i) as *mut char) = '\0';
		});
	    }
	    _			=>	{ 
		putchar(x as char); 
		p_cstr.map( |p| {
		    *(((p as uint)+p_cstr_i) as *mut u8) = x;
		    p_cstr_i += 1;
		    *(((p as uint)+p_cstr_i) as *mut char) = '\0';
		}); 
	    }
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
