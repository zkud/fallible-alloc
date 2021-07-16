use super::layout;
use crate::alloc_error;
use std::alloc::alloc;

pub fn alloc_array<T: Sized>(size: usize) -> Result<*mut T, alloc_error::AllocError> {
    let layout = layout::create_vec_layout::<T>(size)?;

    let array = unsafe { alloc(layout) };

    if array.is_null() {
        Err(alloc_error::AllocError::new(
            format!("Failed to allocate an array with size = {}", size),
            alloc_error::AllocErrorType::FailedAllocation,
        ))
    } else {
        Ok(array as *mut T)
    }
}
