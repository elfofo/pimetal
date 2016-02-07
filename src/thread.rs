
const RPI_SYSTIMER_BASE: u32 = 0x20003000;

#[allow(dead_code)]
struct RpiSysTimer {
	control_status: u32,
	pub counter_lo: u32,
	//counter_hi: u32,
	//compare0: u32,
	//compare1: u32,
	//compare2: u32,
	//compare3: u32,
}


pub fn usleep(us: u32) {
	let t = unsafe { &*(RPI_SYSTIMER_BASE as *const RpiSysTimer)};
	let ts = t.counter_lo;
	while (t.counter_lo - ts) < us {
		unsafe { asm!(""); }
	}
}

pub fn msleep(ms: u32) {
	usleep(ms * 1000)
}
