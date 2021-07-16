//! Vector fallible allocations

use super::alloc_error;
use crate::util::alloc as util_alloc;

/// Allocates a [`Vec<T>`] with given size
///
/// If it's not possible, it will return an [crate::alloc_error::AllocError]
///
/// Usage example
/// ```rust
/// use fallible_alloc::vec::alloc_with_size;
/// let size = 123;
/// match alloc_with_size::<i32>(size) {
///   Ok(vec) => println!("Created a vec with size 10"),
///   Err(error) => println!("Failed to create a vec, reason: {}", error)
/// };
/// ```
pub fn alloc_with_size<T: Sized>(size: usize) -> Result<Vec<T>, alloc_error::AllocError> {
    let buffer = util_alloc::alloc_array(size)?;
    Ok(unsafe { Vec::from_raw_parts(buffer, size, size) })
}
