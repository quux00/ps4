/* io::mod.rs */

use core::option::Some;
use super::drivers;

mod font;

/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/BBABEGGE.html */
pub static UART0: *mut u32 = 0x101f1000 as *mut u32;

pub static UART0_IMSC: *mut u32 = (0x101f1000 + 0x038) as *mut u32;
pub static VIC_INTENABLE: *mut u32 = (0x10140000 + 0x010) as *mut u32;

pub static mut CURSOR_X: u32 = 0;
pub static mut CURSOR_Y: u32 = 0;
pub static CURSOR_HEIGHT: u32 = 16;
pub static CURSOR_WIDTH: u32 = 8;
pub static mut CURSOR_COLOR: u32 = 0x000000FF;
pub static mut FG_COLOR: u32 = 0x00FFFFFF;
pub static mut BG_COLOR: u32 = 0x00000000;
pub static mut CURSOR_BUFFER: [u32, ..8*16] = [0x00FF0000, ..8*16];
pub static mut SAVE_X: u32 = 0;
pub static mut SAVE_Y: u32 = 0;

pub unsafe fn write_char(c: char, address: *mut u32) {
	*address = c as u32;
}

pub unsafe fn scrollup(start: u32)
{
    let mut i = CURSOR_HEIGHT*640;
    while i < (640*480)
    {
	*((start + ((i-16*640)*4)) as *mut u32) = *((start+(i*4)) as *u32); 
	i += 1;
    }
    i = 4*(640*480 - CURSOR_HEIGHT*640);
    while i < 4*640*480
    {
	*((start + (i as u32)) as *mut u32) = BG_COLOR;
	i += 4;
    }
    CURSOR_X = 0x0u32;
    CURSOR_Y -= CURSOR_HEIGHT;
}
pub unsafe fn draw_char(c: char, start: u32)
{
	let font_offset = (c as u8) - 0x20;
	let map = font::bitmaps[font_offset];
	
	let mut i = 0;
	let mut j = 0;
	while j < CURSOR_HEIGHT
	{
		while i < CURSOR_WIDTH
		{
			let addr = start + 4*(CURSOR_X + CURSOR_WIDTH - i + 640*(CURSOR_Y + j));
			if ((map[j] >> 4*i) & 1) == 1
			{
				*(addr as *mut u32) = FG_COLOR;
			}
			else
			{
				*(addr as *mut u32) = BG_COLOR;
			}
			i += 1;
		}
		i = 0;
		j += 1;
	}
}


pub unsafe fn backup(width: u32, start: u32)
{
	let mut i = 0;
	let mut j = 0;
	while j < CURSOR_HEIGHT
	{
		while i < CURSOR_WIDTH
		{
			let addr = start + 4*(CURSOR_X + i + width*(CURSOR_Y + j));
			CURSOR_BUFFER[i + j*8] = *(addr as *mut u32);
			i += 1;
		}
		i = 0;
		j += 1;
	}
	SAVE_X = CURSOR_X;
	SAVE_Y = CURSOR_Y;
}

pub unsafe fn restore(width: u32, start: u32)
{
	let mut i = 0;
	let mut j = 0;
	while j < CURSOR_HEIGHT
	{
		while i < CURSOR_WIDTH
		{
			let addr = start + 4*(SAVE_X + i + width*(SAVE_Y + j));
			*(addr as *mut u32) = CURSOR_BUFFER[i + j*8];
			i += 1;
		}
		i = 0;
		j += 1;
	}
}

pub unsafe fn draw_cursor(width: u32, start: u32)
{
	if (start + 4*(480*640)) <= (start + 4*(640*CURSOR_Y+CURSOR_X+2*CURSOR_WIDTH))
	{
	    scrollup(start);
	}
	
	let mut i = 0;
	let mut j = 0;

	while j < CURSOR_HEIGHT
	{
		while i < CURSOR_WIDTH
		{
			let addr = start + 4*(CURSOR_X + i + width*(CURSOR_Y + j));
			*(addr as *mut u32) = CURSOR_COLOR;
			i += 1;
		}
		i = 0;
		j += 1;
	}
	
}

pub unsafe fn paint(color: u32, start: u32)
{
	let mut i = 0; 
	while i < 640*480
	{
		*((start as u32 + i*4) as *mut u32) = color;
		i+=1;
	}
}

pub unsafe fn fill_bg(start: u32)
{
    paint(BG_COLOR, start);
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

pub unsafe fn set_fg(color: u32)
{
    FG_COLOR = color;
}

pub unsafe fn set_bg(color: u32)
{
    BG_COLOR = color;
}

pub unsafe fn set_cursor_color(color: u32)
{
    CURSOR_COLOR = color;
}
