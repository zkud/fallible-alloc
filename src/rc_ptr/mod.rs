//! Rc fallible allocations

use super::alloc_error;
use crate::util::alloc as util_alloc;
use std::rc::Rc;
use std::mem;

/// Allocates a [`Rc<T>`]
///
/// Usage example
/// ```rust
/// use fallible_alloc::rc_ptr::alloc;
/// match alloc::<i32>() {
///   Ok(value) => println!("Created a rc of i32"),
///   Err(error) => println!("Failed to create a rc, reason: {}", error)
/// };
/// ```
///
/// # Errors
///
/// If allocation is not possible due to issues with memory layouts or not enough memory,
/// it will return an [AllocError](crate::alloc_error::AllocError)
pub fn alloc<T: Sized>() -> Result<Rc<T>, alloc_error::AllocError> {
    let rc_ptr: *mut Rc<T> = util_alloc::alloc_zeroed_value()?;
    Ok((unsafe { &*rc_ptr }).clone())
}
