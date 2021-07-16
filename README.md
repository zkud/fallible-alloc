# Fallible rust stable std collections allocations
[![failable_alloc](https://github.com/zkud/fallible-alloc/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/zkud/fallible-alloc/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/zkud/fallible-alloc/branch/main/graph/badge.svg?token=N2YD1XEW8D)](https://codecov.io/gh/zkud/fallible-alloc)
[![Hits-of-Code](https://hitsofcode.com/github/zkud/fallible-alloc?branch=main)](https://hitsofcode.com/github/zkud/fallible-alloc/view?branch=main)

At the moment we have an unstabilized allocations API in the std,
so this is a temporary safe solution for a stable rust.

## Usage example

To create a vector you could use this code example:
```
use fallible_alloc::vec::alloc_with_size;

...

let vector_size: usize = 10;
let maybe_vector = alloc_with_size::<f64>(vector_size);

match maybe_vector {
  Ok(vec) => println!("Created a vec with size 10"),
  Err(error) => println!("Failed to create a vec, reason: {}", error)
}
```
As you could see, the maybe_vector has a ```Result<Vec<T>, falliable_alloc::AllocError> type```,
so now it's possible to handle a part of allocation errors.

Also it's possible to change the allocator used by crate with this code example:
```
use std::alloc::{GlobalAlloc, System, Layout};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;
```
