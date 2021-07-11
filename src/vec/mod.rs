use super::alloc_error;
use crate::util::alloc as util_alloc;
use std::alloc;

pub fn alloc_with_size<T: Sized, A: alloc::GlobalAlloc>(
    allocator: A,
    size: usize,
) -> Result<Vec<T>, alloc_error::AllocError> {
    let buffer = util_alloc::alloc_array(allocator, size)?;
    Ok(unsafe { Vec::from_raw_parts(buffer, size, size) })
}
