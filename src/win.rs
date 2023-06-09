pub struct Win {
	dev: ::Device,
	pub win: u64,

	#[allow(dead_code)]
	x_res: u32,

	#[allow(dead_code)]
	y_res: u32,
}

pub type WinDrawHook = extern "C" fn(win: u64, data: u64) -> u64;

impl Win {
	pub fn new(x_res: u32, y_res: u32) -> Win {
		let dev = ::query_device("aquabsd.alps.win");
		let win = ::send_device!(dev, 0x6377, x_res, y_res);

		Win { dev, win, x_res, y_res }
	}

	pub fn caption(&mut self, name: &str) {
		let c_str = std::ffi::CString::new(name).unwrap();
		::send_device!(self.dev, 0x7363, self.win, c_str.as_ptr());
	}

	pub fn draw_hook(&mut self, hook: WinDrawHook, data: u64) {
		::send_device!(self.dev, 0x7263, self.win, 0, hook, data);
	}

	pub fn draw_loop(&mut self) {
		::send_device!(self.dev, 0x6C6F, self.win);
	}
}

impl Drop for Win {
	fn drop(&mut self) {
		::send_device!(self.dev, 0x6463, self.win);
	}
}
