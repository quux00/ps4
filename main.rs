#[link(name = "ironkernel",
       vers = "0.1",
       license = "MIT")];
// Forked from pczarn/rustboot
#[no_std];
#[feature(asm, globs, macro_rules)];

use core::slice::iter; // for the iterator
use core::iter::Iterator; // for the loop
use core::option::{Some, Option, None}; // for the loop
use core::str::*;
use core::vec;

use platform::*;

#[path = "rust-core/core/mod.rs"]
mod core;

mod kernel {
    pub mod int;
    pub mod memory;
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

fn keydown(key: char) {
    unsafe {
        io::write_char(key);
    }
}

/// Name courtesy of John 
unsafe fn suckit(key: u8) {
    drivers::keydown.map(|f| {
        f(key as char);
    });
}

/// Name also courtesy of John
#[lang = "exchange_free"]
unsafe fn blow_me(msg: &str) {
    for c in core::slice::iter(as_bytes(msg)) {
        suckit(*c);
    }
}

#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    io::keydown(keydown);
    let table = cpu::interrupt::table::new();
    table.load();
    drivers::init(table);
    suckit('t' as u8);
    suckit('e' as u8);
    blow_me(&"piss");
    suckit(8 as u8);
    suckit(8 as u8);
    suckit(8 as u8);
    suckit(8 as u8);
    blow_me(&"workworkwork");
}
