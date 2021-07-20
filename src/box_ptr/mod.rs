//! Box fallible allocations

use super::alloc_error;
use crate::util::alloc as util_alloc;

/// Allocates a [`Box<T>`]
///
/// Usage example
/// ```rust
/// use fallible_alloc::box_ptr::alloc;
/// match alloc::<i32>() {
///   Ok(value) => println!("Created a box of i32"),
///   Err(error) => println!("Failed to create a box, reason: {}", error)
/// };
/// ```
///
/// # Errors
///
/// If allocation is not possible due to issues with memory layouts or not enough memory,
/// it will return an [AllocError](crate::alloc_error::AllocError)
pub fn alloc<T: Sized>() -> Result<Box<T>, alloc_error::AllocError> {
    let value_ptr = util_alloc::alloc_value()?;
    Ok(unsafe { Box::from_raw(value_ptr) })
}