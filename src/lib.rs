//! # Fallible rust stable std collections allocations
//!
//! While the failable allocation API for std is being stabilized
//! there should be a solution, so that's why this crate exists.
//!
//! # Usage example
//!
//! To create a vector you could use this code example:
//! ```rust
//! use fallible_alloc::vec::alloc_with_size;
//!
//! let vector_size: usize = 10;
//! let maybe_vector = alloc_with_size::<f64>(vector_size);
//!
//! match maybe_vector {
//!   Ok(vec) => println!("Created a vec with size 10"),
//!   Err(error) => println!("Failed to create a vec, reason: {}", error)
//! }
//! ```
//! As you could see, the maybe_vector has a Result<[`Vec<T>`], [AllocError](crate::alloc_error::AllocError)> type,
//! so now it's possible to handle a part of allocation errors.
//!
//! Also it's possible to change the allocator used by crate with this code example:
//! ```rust
//! use std::alloc::{GlobalAlloc, System, Layout};
//!
//! struct MyAllocator;
//!
//! unsafe impl GlobalAlloc for MyAllocator {
//!     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//!         System.alloc(layout)
//!     }
//!
//!     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//!         System.dealloc(ptr, layout)
//!     }
//! }
//!
//! #[global_allocator]
//! static GLOBAL: MyAllocator = MyAllocator;
//! ```

#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]
#![doc(html_root_url = "https://docs.rs/fallible_alloc/0.3.0")]

pub mod alloc_error;
pub mod box_ptr;
pub mod rc_ptr;
mod util;
pub mod vec;
