// This Source Form is subject to the terms of the AQUA Software License, v. 1.0.
// Copyright (c) 2023 Aymeric Wibo
// Copyright (c) 2023 Alexis Englebert

// modules which are always there

/// Module for creating, manipulating, and interacting with windows.
///
/// # Examples
///
/// ```
/// let mut win = aqua::win::Win::new(800, 600);
/// win.caption("aqua::win example");
///
/// win.draw_loop(|| {
///    0
/// });
/// ```
pub mod win;

/// Module for reading mouse input for physical mice as well as virtual (window-relative) mice.
pub mod mouse;

/// Module for reading the properties and raw image data of PNG images.
pub mod png;

// modules which are only there for certain features

/// Module for accessing the Vulkan API.
#[cfg(feature = "vk")]
pub mod vk;

extern {
	/// Entry point for the KOS.
	/// Shouldn't be called directly; you must define it yourself.
	///
	/// # Examples
	///
	/// ```
	/// #[no_mangle]
	/// pub fn main(void) {
	///    println!("Hello world!");
	/// }
	/// ```
	#[link_name="main"]
	pub fn main();
}

#[no_mangle]
extern "C" fn __native_entry() {
	unsafe { main() };
}

/// Device ID type, returned by `query_device`.
pub type Device = u64;

type KosQueryDevice = fn(u64, u64) -> Device;
type KosSendDevice = fn(u64, u64, u64, u64) -> u64;

static mut KOS_QUERY_DEVICE: Option<KosQueryDevice> = None;
static mut KOS_SEND_DEVICE: Option<KosSendDevice> = None;

// called by the KOS to set query_device & send_device

#[no_mangle]
unsafe extern "C" fn aqua_set_kos_functions(kos_query_device: u64, kos_send_device: u64) {
	KOS_QUERY_DEVICE = Some(std::mem::transmute(kos_query_device));
	KOS_SEND_DEVICE = Some(std::mem::transmute(kos_send_device));
}

// wrappers around C function pointers given to us by the KOS

/// Query a device ID by name.
///
/// # Arguments
///
/// * `name`: The name of the device.
///
/// # Examples
///
/// ```
/// let dev = aqua::query_device("aquabsd.alps.win");
/// ```
pub fn query_device(name: &str) -> Device {
	let c_str = std::ffi::CString::new(name).unwrap();

	unsafe {
		KOS_QUERY_DEVICE.unwrap()(0, c_str.as_ptr() as u64)
	}
}

/// Send a command to a device.
///
/// # Arguments
///
/// * `device`: The device to send the command to.
/// * `cmd`: The command to send the device.
/// * `data`: Extra data to send the device along with the command.
///
/// # Examples
///
/// ```
/// let dev = aqua::query_device("aquabsd.alps.win");
/// let win = aqua::raw_send_device(dev, 0x6377, &mut [800, 600]);
/// ```
pub fn raw_send_device(device: Device, cmd: u16, data: &mut [u64]) -> u64 {
	unsafe {
		KOS_SEND_DEVICE.unwrap()(0, device as u64, cmd as u64, data.as_ptr() as u64)
	}
}

/// Wrapper around `raw_send_device` to make it a little more ergonomic to use.
///
/// # Arguments
///
/// * `device`: The device to send the command to.
/// * `cmd`: The command to send the device.
/// * `data`: Extra data to send the device along with the command.
///
/// # Examples
///
/// ```
/// let dev = aqua::query_device("aquabsd.alps.win");
/// let win = aqua::send_device!(dev, 0x6377, 800, 600);
/// ```
#[macro_export]
macro_rules! send_device {
	($device: expr, $cmd: expr, $($data: expr),*) => {
		(|| -> u64 {
			let mut data = [$($data as u64),*];
			::raw_send_device($device, $cmd as u16, &mut data)
		})()
	};
}
