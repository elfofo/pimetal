//kernel.rs
#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]

mod gpio;
mod thread;
use gpio::Pin;
use thread::msleep;

const LED_OK: u32 = 16;


#[no_mangle]
pub extern fn _start() {
	main()
}

#[no_mangle]
pub extern fn main() {

	let mut pin = Pin::new(LED_OK);

	pin.as_out();

	loop {
		msleep(500);
		pin.on();
		msleep(500);
		pin.off();
	}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}

