use crate::alloc_error;
use std::alloc;

pub fn create_vec_layout<T>(size: usize) -> Result<alloc::Layout, alloc_error::AllocError> {
    Ok(alloc::Layout::array::<T>(size)?)
}
