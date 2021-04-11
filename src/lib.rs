//! TODO
#![deny(rust_2018_idioms, missing_docs, clippy::pedantic)]

pub mod sys;

#[doc(hidden)]
pub mod __private {
    pub use startup::on_startup;
}

/// Register one, or multiple MMIO plugins, that will be enabled when the library is loaded.
///
/// # Example
///
/// ```ignore
/// pub struct MyPlugin {
///     // ...
/// }
///
/// impl spike_rs::Plugin for MyPlugin {
///     // ...
/// }
///
/// spike_rs::register_plugin!("my_awesome_mmio_plugin" => MyPlugin);
/// ```
#[macro_export]
macro_rules! register_plugins {
    ($name:expr => $plugin:ty $(,)?) => {
        $crate::__private::on_startup! {
            let _: &'static ::std::primitive::str = $name;
            let name: &'static ::std::ffi::CStr = ::std::ffi::CStr::from_bytes_with_nul(
                ::std::concat!($name, "\0").as_bytes()
            ).unwrap();
            let name: *const ::std::os::raw::c_char = name.as_ptr();

            unsafe extern "C" fn alloc(args: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void {
                let args = ::std::ffi::CStr::from_ptr(args).to_string_lossy();
                ::std::boxed::Box::into_raw(
                    ::std::boxed::Box::new(<$plugin as $crate::Plugin>::new(args.as_ref()))
                ).cast::<::std::os::raw::c_void>()
            }

            unsafe extern "C" fn dealloc(this: *mut ::std::os::raw::c_void) {
                let _drop = ::std::boxed::Box::<$plugin>::from_raw(this.cast());
            }

            unsafe extern "C" fn store(
                this: *mut ::std::os::raw::c_void,
                addr: ::std::primitive::u64,
                len: ::std::os::raw::c_ulong,
                buf: *const ::std::primitive::u8,
            ) -> bool {
                let this = &mut *this.cast::<$plugin>();
                let buf = ::std::slice::from_raw_parts(buf, len as usize);
                <$plugin as $crate::Plugin>::store(this, addr, buf)
            }

            unsafe extern "C" fn load(
                this: *mut ::std::os::raw::c_void,
                addr: ::std::primitive::u64,
                len: ::std::os::raw::c_ulong,
                buf: *mut ::std::primitive::u8,
            ) -> bool {
                let this = &mut *this.cast::<$plugin>();
                let buf = ::std::slice::from_raw_parts_mut(buf, len as usize);
                <$plugin as $crate::Plugin>::load(this, addr, buf)
            }

            let plugin: $crate::sys::MmioPlugin = $crate::sys::MmioPlugin {
                alloc,
                dealloc,
                store,
                load,
            };

            unsafe {
                $crate::sys::register_mmio_plugin(name, &plugin);
            }
        }
    };
}

/// High Level trait for defining a MMIO plugin.
pub trait Plugin {
    /// Create a new instance of this plugin, with the arguments
    /// that were provided.
    fn new(args: &str) -> Self;

    /// Fill the whole `buf` with bytes that are located at the given offset.
    ///
    /// Returns `true` if the load succeeded, otherwise `false.
    fn load(&mut self, offset: u64, buf: &mut [u8]) -> bool;

    /// Store every byte from `buf` into the memory at `offset`.
    ///
    /// Returns `true` if the store succeeded, otherwise `false.
    fn store(&mut self, offset: u64, buf: &[u8]) -> bool;
}
