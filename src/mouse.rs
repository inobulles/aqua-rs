pub enum MouseButton {
	Left,
	Right,
	Middle,
}

impl MouseButton {
	fn to_c(&self) -> u64 {
		match self {
			Self::Left => 0,
			Self::Right => 1,
			Self::Middle => 2,
		}
	}
}

pub enum MouseAxis {
	X, Y, Z
}

impl MouseAxis {
	fn to_c(&self) -> u64 {
		match self {
			Self::X => 0,
			Self::Y => 1,
			Self::Z => 2,
		}
	}
}

pub struct Mouse {
	dev: ::Device,
	mouse: u64,
}

impl Mouse {
	pub fn default() -> Mouse {
		let dev = ::query_device("aquabsd.alps.mouse");
		let mouse = ::send_device!(dev, 0x646D,);

		Mouse { dev, mouse }
	}

	pub fn update(&mut self) {
		::send_device!(self.dev, 0x756D, self.mouse);
	}

	pub fn poll_button(&mut self, button: MouseButton) -> bool {
		::send_device!(self.dev, 0x7062, self.mouse, button.to_c()) != 0
	}

	pub fn poll_axis(&mut self, axis: MouseAxis) -> f32 {
		let raw = ::send_device!(self.dev, 0x7061, self.mouse, axis.to_c());
		unsafe { std::mem::transmute(raw as u32) }
	}
}
