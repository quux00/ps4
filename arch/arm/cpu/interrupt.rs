use platform::io;
use support::memset;

struct table;

impl table {
    pub unsafe fn new() -> table {
        table
    }

    pub unsafe fn enable(&self, irq: u8, isr: u32) {
        // b isr
	// See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0203j/Cihdidh2.html
        *((irq * 4) as *mut u32) =
            0xea000000 | ((isr - irq as u32 * 4 - 8) >> 2);
    }

    pub unsafe fn load(&self) {
        asm!("mov r2, sp
          mrs r0, cpsr
          bic r1, r0, #0x1F
          orr r1, r1, #0x12
          msr cpsr, r1
          mov sp, 0x19000
          bic r0, r0, #0x80
          msr cpsr, r0
          mov sp, r2"
        ::: "r0", "r1", "r2", "cpsr");

	/* 
	 * See
	 * http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/I1042232.html
	 * and pczarn's comment at 
	 * https://github.com/wbthomason/ironkernel/commit/4b199b502b2fc5d42b7f1571b52dd1b0c657e77b#arch-arm-cpu-interrupt-rs-P6
	 */
        *io::VIC_INTENABLE = 1 << 12;

	/*
	 * See
	 * http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0183f/I54603.html
	 */
        *io::UART0_IMSC = 1 << 4;

	// not sure what this is doing
        let mut i = 0;
        while i < 10 {
            *((i*4) as *mut u32) = vectors[i];
            i += 1;
        }
    }
}

// not sure what this is
extern {
    static vectors: [u32, ..10];
}

/*
#[lang="fail_"]
#[fixed_stack_segment]
pub fn fail(expr: *u8, file: *u8, line: uint) -> ! {
    unsafe { zero::abort(); }
}

#[lang="fail_bounds_check"]
#[fixed_stack_segment]
pub fn fail_bounds_check(file: *u8, line: uint, index: uint, len: uint) {
    unsafe { zero::abort(); }
}
*/
