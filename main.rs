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
	wtm(0x10000010, 0x2CAC);
	wtm(0x10120000, 0x1313A4C4);
	wtm(0x10120004, 0x0505F657);
	wtm(0x10120008, 0x071F1800);
*/
	// 640x480
	// See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CACCCFBF.html
	wtm(0x10000010, 0x2C77);

	wtm(0x10120000, 0x3F1F3F9C);
	wtm(0x10120004, 0x090B61DF);
	wtm(0x10120008, 0x067F1800);

	/* See http://forum.osdev.org/viewtopic.php?p=195000 */
	wtm(0x10120010, (1*1024*1024));

	/* See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0161e/I911024.html */
	wtm(0x10120018, 0x82B);
	
	// This doesn't seem to do anything
	// wtm(0x10008000, 0xF0F0F0F0);
	
	let pl = (1024*1024) as *mut u32;
	let mut i = 0; 
	while i < 640
	{
		// not sure about this color scheme
		*((pl as u32 + 50*i) as *mut u32) = 0x00FFFFFF;
		i+=1;
	}
}

pub unsafe fn wtm(addr: u32, value: u32)
{
	*(addr as *mut u32) = value;
}
