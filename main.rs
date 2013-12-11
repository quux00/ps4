/* main.rs */

#[link(name = "ironkernel",
       vers = "0.1",
       license = "MIT")];
// Forked from pczarn/rustboot
#[no_std];
#[feature(asm, globs, macro_rules)];

use core::option::{Some, Option, None}; // for the loop

use platform::*;
use kernel::*;

#[path = "rust-core/core/mod.rs"]
mod core;

mod kernel {
    pub mod int;
    pub mod memory;
    pub mod sgash;
}


#[cfg(target_arch = "arm")]
#[path = "arch/arm/"]
mod platform {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
}

#[cfg(target_arch = "arm")]
#[path = "rust-core/support.rs"]
mod support;


#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
	drivers::keydown = Some(sgash::parsekey);
	let table = cpu::interrupt::table::new();
	table.load();
	drivers::init(table);
	sgash::init(); 
	/* For the following magic values, see 
	 * http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CACHEDGD.html
	 */
/*
	// 800x600
	// See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CACCCFBF.html
	ws(0x10000010, 0x2CAC);
	ws(0x10120000, 0x1313A4C4);
	ws(0x10120004, 0x0505F657);
	ws(0x10120008, 0x071F1800);
*/
	// 640x480
	// See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CACCCFBF.html
	ws(0x10000010, 0x2C77);

	ws(0x10120000, 0x3F1F3F9C);
	ws(0x10120004, 0x090B61DF);
	ws(0x10120008, 0x067F1800);

	/* See http://forum.osdev.org/viewtopic.php?p=195000 */
	ws(0x10120010, (1*1024*1024));

	/* See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0161e/I911024.html */
	ws(0x10120018, 0x82B);
/*	
 *	Turns out this board using the 110 Primecell, not the 111 one. So we can't do stuff with a
 *	cursor.
 */
	/*
	let mut i = 0; 
	while i < 0x3FC
	{
		wh(0x10120800 + i, 0xFFFFFFFF);
		i += 1;
	}
	ws(0x10120C00, 0x00000002);
	ws(0x10120C04, 0x00000001);
	ws(0x10120C20, 0x00000001);
	*/
}

pub unsafe fn paint(color: u32)
{
	let mut i = 0; 
	let pl = (1024*1024) as *mut u32;
	while i < 640*480
	{
		*((pl as u32 + i*4) as *mut u32) = color;
		i+=1;
	}
}

pub unsafe fn read(addr: u32)	->	u32
{
	*(addr as *mut u32)
}

pub unsafe fn ws(addr: u32, value: u32)
{
	*(addr as *mut u32) = *(addr as *mut u32) | value;
}

pub unsafe fn wh(addr: u32, value: u32)
{
	*(addr as *mut u32) = value;
}
