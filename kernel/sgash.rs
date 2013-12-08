/* kernel::sgash.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::vec::Vec;
use core::mem::Allocator;
use kernel::*;
use super::super::platform::*;

pub static mut buffer: cstr = cstr {
				p: 0 as *mut u8,
				p_cstr_i: 0,
				max: 0
			      };
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

/*
pub unsafe fn output(&cstr)
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
*/

pub unsafe fn parsekey(x: char) {
    let x = x as u8;
    // Set this to false to learn the keycodes of various keys!
    // Key codes are printed backwards because life is hard
    if (true) {
	match x {
	    13		=>	{ prompt(); }
	    127		=>	{ 
		if (buffer.delete_char()) { putchar(''); }
	    }
	    _		=>	{ 
		if (buffer.add_char(x)) { putchar(x as char); }
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
    buffer = cstr::new(256);
}

unsafe fn prompt() {
    putstr(&"\nsgash > ");
    buffer.reset();
}

/* BUFFER MODIFICATION FUNCTIONS */

struct cstr {
    p: *mut u8,
    p_cstr_i: uint,
    max: uint 
}

impl cstr {
    pub unsafe fn new(size: uint) -> cstr {
	let (x, y) = memory::allocator.alloc(size);
	let this = cstr {
	    p: x,
	    p_cstr_i: 0,
	    max: y
	};
	this
    }

    unsafe fn add_char(&mut self, x: u8) -> bool{
	if (self.p_cstr_i == self.max) { return false; }
	*(((self.p as uint)+self.p_cstr_i) as *mut u8) = x;
	self.p_cstr_i += 1;
	*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
	true
    }

    unsafe fn delete_char(&mut self) -> bool {
	if (self.p_cstr_i == 0) { return false; }
	self.p_cstr_i -= 1;
	*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
	true
    }

    unsafe fn reset(&mut self) {
	self.p_cstr_i = 0; 
	*(self.p as *mut char) = '\0';
    }
}

/* Utility functions! */
/*
fn eq(a: *mut u8, b: *mut u8) {
    ind = 0;
    while (a[i] != 
}
*/
