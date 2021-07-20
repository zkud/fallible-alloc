//! Rc fallible allocations

use super::alloc_error;
use crate::util::alloc as util_alloc;
use std::rc::Rc;

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
    let value_ptr = util_alloc::alloc_value()?;
    Ok(unsafe { Rc::from_raw(value_ptr) })
}

