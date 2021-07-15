use std::alloc::GlobalAlloc;
use std::alloc::System;
use std::alloc::Layout;
use std::ptr;

use fallible_alloc::alloc_error::AllocErrorType;

/**
 * A simple allocator for simulation of causing oom environment
 */
struct SmallMemoryAllocator;

unsafe impl GlobalAlloc for SmallMemoryAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() > 30000 {
            ptr::null_mut()
        } else {
            System.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: SmallMemoryAllocator = SmallMemoryAllocator;

#[test]
fn with_enough_mem_it_allocates() {
    let vec: Vec<f64> = fallible_alloc::vec::alloc_with_size(10).unwrap();
    assert_eq!(vec.len(), 10);
}

#[test]
fn with_not_enough_mem_it_handles_oom() {
    match fallible_alloc::vec::alloc_with_size::<f64>(5000000000) {
        Ok(_) => panic!("There should be an error, when unable to allocate memory"),
        Err(error) => {
            assert_eq!(error.error_type(), AllocErrorType::FailedAllocation);
        }
    }
}