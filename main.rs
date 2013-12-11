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
	io::ws(0x10000010, 0x2CAC);
	io::ws(0x10120000, 0x1313A4C4);
	io::ws(0x10120004, 0x0505F657);
	io::ws(0x10120008, 0x071F1800);
*/
	// 640x480
	// See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CACCCFBF.html
	io::ws(0x10000010, 0x2C77);

	io::ws(0x10120000, 0x3F1F3F9C);
	io::ws(0x10120004, 0x090B61DF);
	io::ws(0x10120008, 0x067F1800);

	/* See http://forum.osdev.org/viewtopic.php?p=195000 */
	io::ws(0x10120010, (1*1024*1024));

	/* See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0161e/I911024.html */
	io::ws(0x10120018, 0x82B);

	io::paint(0x00FF0000, 1024*1024);
	io::draw_cursor(640, 1024*1024);
}

