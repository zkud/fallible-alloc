use crate::alloc_error;
use std::alloc;

#[inline]
pub fn alloc_array<T: Sized>(size: usize) -> Result<*mut T, alloc_error::AllocError> {
    let layout = alloc::Layout::array::<T>(size)?;

    let array = unsafe { alloc::alloc(layout) };

    if array.is_null() {
        Err(alloc_error::AllocError::new(
            format!("Failed to allocate an array with size = {}", size),
            alloc_error::AllocErrorType::FailedAllocation,
        ))
    } else {
        Ok(array as *mut T)
    }
}

#[inline]
pub fn alloc_value<T: Sized>() -> Result<*mut T, alloc_error::AllocError> {
    let layout = alloc::Layout::new::<T>();

    let value_ptr = unsafe { alloc::alloc(layout) };

    if value_ptr.is_null() {
        Err(alloc_error::AllocError::new(
            "Failed to allocate a value",
            alloc_error::AllocErrorType::FailedAllocation,
        ))
    } else {
        Ok(value_ptr as *mut T)
    }
}

#[inline]
pub fn alloc_zeroed_value<T: Sized>() -> Result<*mut T, alloc_error::AllocError> {
    let layout = alloc::Layout::new::<T>();

    let value_ptr = unsafe { alloc::alloc_zeroed(layout) };

    if value_ptr.is_null() {
        Err(alloc_error::AllocError::new(
            "Failed to allocate a value",
            alloc_error::AllocErrorType::FailedAllocation,
        ))
    } else {
        Ok(value_ptr as *mut T)
    }
}
