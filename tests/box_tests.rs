use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::alloc::System;
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

struct SmallType([u128; 10]);
struct BigType([u128; 100000000000]);

#[test]
fn with_enough_mem_it_allocates() {
    fallible_alloc::box_ptr::alloc::<SmallType>().unwrap();
}

#[test]
fn with_not_enough_mem_it_handles_oom() {
    match fallible_alloc::box_ptr::alloc::<BigType>() {
        Ok(_) => panic!("There should be an error, when unable to allocate memory"),
        Err(error) => {
            assert_eq!(error.error_type(), AllocErrorType::FailedAllocation);
            assert_eq!(error.message(), "Failed to allocate a value");
        }
    }
}
