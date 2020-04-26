//! Macros for Kernel-Mode drivers.

/// Find field offset in a struct
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        //  Undefined Behavior: dereferences a null pointer.
        //  Undefined Behavior: accesses field outside of valid memory area.
        #[allow(unused_unsafe)]
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize }
    }
}

/// Macro to send a message to the kernel debugger.
///
/// # Example
///
/// ```no_run
/// KdPrint!("NTSTATUS is 0x%X\n", status);
/// ```
#[macro_export]
macro_rules! KdPrint {
	($msg:expr $(, $arg:expr)*) => { #[allow(unused_unsafe)] unsafe { $crate::debug::DbgPrint( concat!($msg, "\n\0").as_ptr() $(, $arg )* )} };
}

/// Macro to send a message to the kernel debugger for unsafe blocks.
///
/// Used in `unsafe {}` blocks.
#[macro_export]
macro_rules! KdPrint_u {
	($msg:expr $(, $arg:expr)*) => { $crate::debug::DbgPrint( concat!($msg, "\n\0").as_ptr() $(, $arg )* ) };
}

#[macro_export]
macro_rules! check_unsafe {
	($expr:expr) => {{
		let st: $crate::status::Status = unsafe { $expr };
		if st.is_err() {
			KdPrint!("[km] error: status 0x%X\n", st);
			return st;
		} else {
			st
		}
	}}
}
