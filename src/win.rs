// This Source Form is subject to the terms of the AQUA Software License, v. 1.0.
// Copyright (c) 2023 Aymeric Wibo
// Copyright (c) 2023 Alexis Englebert

enum Cmd {
	Create     = 0x6377,
	Delete     = 0x6477,
	SetCaption = 0x7363,
	RegisterCb = 0x7263,
	Loop       = 0x6C6F,
}

enum Cb {
	DRAW,
}

type WinDrawHook = unsafe extern "C" fn(win: u64, data: u64) -> u64;

/// An `aquabsd.alps.win` window object.
pub struct Win {
	dev: ::Device,
	pub win: u64,
}

// thanks to https://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/

unsafe extern "C" fn draw_trampoline<F>(_win: u64, data: u64) -> u64 where
	F: FnMut() -> u64,
{
	let closure = &mut *(data as *mut F);
	closure()
}

fn get_draw_trampoline<F>(_closure: &F) -> WinDrawHook where
	F: FnMut() -> u64,
{
	draw_trampoline::<F>
}

impl Win {
	/// Create a new window with the given resolution.
	///
	/// # Arguments
	///
	/// * `x_res` - Horizontal resolution of the window, in pixels.
	/// * `y_res` - Vertical resolution of the window, in pixels.
	///
	/// # Examples
	///
	/// ```
	/// let win = aqua::win::Win::new(800, 600);
	/// ```
	pub fn new(x_res: u32, y_res: u32) -> Self {
		let dev = ::query_device("aquabsd.alps.win");
		let win = ::send_device!(dev, Cmd::Create, x_res, y_res);

		Win { dev, win }
	}

	/// Set the caption of the window.
	///
	/// # Arguments
	///
	/// * `name` - Caption to be given.
	///
	/// # Examples
	///
	/// ```
	/// let mut win = aqua::win::Win::new(800, 600);
	/// win.caption("This is a Caption");
	/// ```
	pub fn caption(&mut self, name: &str) {
		let c_str = std::ffi::CString::new(name).unwrap();
		::send_device!(self.dev, Cmd::SetCaption, self.win, c_str.as_ptr());
	}

	/// Starts the draw loop of the window.
	///
	/// # Arguments
	///
	/// * `closure` - Closure to be called each time the window is drawn.
	///
	/// # Examples
	///
	/// ```
	/// let mut win = aqua::win::Win::new(800, 600);
	/// win.draw_loop(|| {
	///    0
	/// });
	/// ```
	pub fn draw_loop<F>(&mut self, closure: F) where
		F: FnMut() -> u64,
	{
		unsafe {
			let trampoline = get_draw_trampoline(&closure);
			let data: u64 = std::mem::transmute(&closure);
			::send_device!(self.dev, Cmd::RegisterCb, self.win, Cb::DRAW, trampoline, data);
		}

		::send_device!(self.dev, Cmd::Loop, self.win);
	}
}

impl Drop for Win {
	fn drop(&mut self) {
		::send_device!(self.dev, Cmd::Delete, self.win);
	}
}
