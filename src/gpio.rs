
const GPIO_BASE: u32 = 0x20200000;
const GPF_SET: isize = 7;
const GPF_CLR: isize = 10;

pub struct Pin {
	pub num: u32,
	fun_sel: *mut u32,
	bit_sel: u32,
	fun_set: *mut u32,
	fun_clr: *mut u32,
	bit_set_clr: u32,
}

#[allow(dead_code)]
impl Pin {
	pub fn new(pin: u32) -> Pin {
		let base = GPIO_BASE as *mut u32;
		let ipin = pin as isize;
		Pin {
			num: pin,
			fun_sel: unsafe { base.offset(0x04 * (ipin/10)) as *mut u32 },
			bit_sel: (pin % 10) * 3, 
			fun_set: unsafe { base.offset(GPF_SET + (ipin/32)) as *mut u32 },
			fun_clr: unsafe { base.offset(GPF_CLR + (ipin/32)) as *mut u32 },
			bit_set_clr: pin % 32,
		}
	}
	
	pub fn as_out(&mut self) {
		unsafe { *(self.fun_sel) |= 1 << self.bit_sel; }
	}
	pub fn as_in(&mut self) {
		unsafe { *(self.fun_sel) |= 0 << self.bit_sel; }
	}
	pub fn on(&mut self) {
		unsafe { *(self.fun_set) = 1 << self.bit_set_clr; }
	}
	pub fn off(&mut self) {
		unsafe { *(self.fun_clr) = 1 << self.bit_set_clr; }
	}
	pub fn set(&mut self, is_on: bool) {
		match is_on {
			true => self.on(),
			false => self.off(),
		}
	}
}


