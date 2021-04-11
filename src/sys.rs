//! The raw FFI definitions providing unsafe access to the APIs provided by Spike.

use std::os::raw::{c_char, c_ulong, c_void};

/// The `alloc` function is called when a new instance of a plugin is created.
///
/// # Parameters
///
/// - `args`: The argument provided by the user via the command line option.
///
/// # Returns
///
/// A pointer to an allocated instance of this plugin,
/// which is then passed to the other functions.
pub type AllocFunction =
    unsafe extern "C" fn(args: *const c_char) -> *mut c_void;

/// The `load` function loads a region of memory from this MMIO plugin.
///
/// # Parameters
///
/// - `this`: The allocated instance of the plugin.
/// - `addr`: The offset from which to load the memory from.
/// - `len`: Number of bytes to read.
/// - `buf`: The buffer in which the read memory must be stored.
///
/// # Returns
///
/// `true` if the load was successful.
pub type LoadFunction = unsafe extern "C" fn(
    this: *mut c_void,
    addr: u64,
    len: c_ulong,
    buf: *mut u8,
) -> bool;

/// The `store` function writes a region of memory into this MMIO plugin.
///
/// # Parameters
///
/// - `this`: The allocated instance of the plugin.
/// - `addr`: The offset to store the data into.
/// - `len`: Number of bytes to write.
/// - `buf`: The buffer in which the bytes to store are located.
///
/// # Returns
///
/// `true` if the store was successful.
pub type StoreFunction = unsafe extern "C" fn(
    this: *mut c_void,
    addr: u64,
    len: c_ulong,
    buf: *const u8,
) -> bool;

/// The `dealloc` function is called when the plugin is deallocated / disabled.
///
/// # Parameters
///
/// - `this`: The instance of a plugin which should be deallocated.
pub type DeallocFunction = unsafe extern "C" fn(this: *const c_void);

/// The raw FFI-compatible representation of a MMIO plugin.
#[repr(C)]
pub struct MmioPlugin {
    /// The `alloc` function of this plugin.
    pub alloc: AllocFunction,
    /// The `load` function of this plugin.
    pub load: LoadFunction,
    /// The `store` function of this plugin.
    pub store: StoreFunction,
    /// The `dealloc` function of this plugin.
    pub dealloc: DeallocFunction,
}

extern "C" {
    /// Register a MMIO plugin with the given name.
    pub fn register_mmio_plugin(name: *const c_char, plugin: *const MmioPlugin);
}
