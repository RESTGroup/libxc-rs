//! Utility functions for libxc.

use libxc_ffi::ffi;
use std::ffi::CStr;

unsafe fn cstr_to_string(ptr: *const std::ffi::c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

/// Returns the libxc version as (major, minor, micro).
pub fn libxc_version() -> (i32, i32, i32) {
    let mut major: std::ffi::c_int = 0;
    let mut minor: std::ffi::c_int = 0;
    let mut micro: std::ffi::c_int = 0;
    unsafe {
        ffi::xc_version(&mut major, &mut minor, &mut micro);
    }
    (major as i32, minor as i32, micro as i32)
}

/// Returns the libxc version as a string.
pub fn libxc_version_string() -> String {
    unsafe { cstr_to_string(ffi::xc_version_string()) }
}

/// Returns the literature reference for libxc.
pub fn libxc_reference() -> String {
    unsafe { cstr_to_string(ffi::xc_reference()) }
}

/// Returns the DOI for the libxc reference.
pub fn libxc_reference_doi() -> String {
    unsafe { cstr_to_string(ffi::xc_reference_doi()) }
}

/// Returns the reference key for libxc.
pub fn libxc_reference_key() -> String {
    unsafe { cstr_to_string(ffi::xc_reference_key()) }
}

/// Returns the functional number for a given name, or `None` if not found.
pub fn libxc_functional_get_number(name: &str) -> Option<i32> {
    let c_name = std::ffi::CString::new(name).ok()?;
    let n = unsafe { ffi::xc_functional_get_number(c_name.as_ptr()) };
    if n == -1 {
        None
    } else {
        Some(n as i32)
    }
}

/// Returns the functional name for a given number, or `None` if not found.
pub fn libxc_functional_get_name(number: i32) -> Option<String> {
    let ptr = unsafe { ffi::xc_functional_get_name(number as std::ffi::c_int) };
    if ptr.is_null() {
        return None;
    }
    let s = unsafe { cstr_to_string(ptr) };
    // xc_functional_get_name allocates memory that must be freed
    unsafe {
        libc::free(ptr as *mut std::ffi::c_void);
    }
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Returns the total number of available functionals.
pub fn libxc_number_of_functionals() -> i32 {
    unsafe { ffi::xc_number_of_functionals() as i32 }
}

/// Returns a sorted list of all available functional numbers.
pub fn libxc_available_functional_numbers() -> Vec<i32> {
    let n = libxc_number_of_functionals();
    let mut list = vec![0 as std::ffi::c_int; n as usize];
    unsafe {
        ffi::xc_available_functional_numbers(list.as_mut_ptr());
    }
    list.into_iter().collect()
}

/// Returns a list of all available functional names.
pub fn libxc_available_functional_names() -> Vec<String> {
    let n = libxc_number_of_functionals();
    // Build a contiguous buffer for the name pointers
    let mut name_ptrs = vec![std::ptr::null_mut::<std::ffi::c_char>(); n as usize];
    unsafe {
        ffi::xc_available_functional_names(name_ptrs.as_mut_ptr());
    }
    name_ptrs.into_iter().map(|ptr| unsafe { cstr_to_string(ptr) }).collect()
}

/// Print the library path of dynamic loading / static linking.
pub fn print_library_path() {
    #[cfg(feature = "dynamic_loading")]
    {
        println!("libxc is dynamically loaded from: {:?}", unsafe {
            &ffi::dyload_lib().__libraries_path
        });
    }
    #[cfg(not(feature = "dynamic_loading"))]
    {
        println!("libxc is statically linked, please manually use ldd the binary to check the libxc path.");
    }
}
