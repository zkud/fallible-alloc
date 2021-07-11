# Fallible rust stable std collections allocations
[![failable_alloc](https://github.com/zkud/fallible-alloc/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/zkud/fallible-alloc/actions/workflows/ci.yml)

At the moment we have an unstabilized allocations API in the std,
so this is a temporary safe solution for a stable rust.

## Usage example

To create a vector you could use this code example:
```
use fallible_alloc::vec::alloc_with_size;
use std::alloc::System;

...

let vector_size: usize = 10;
let maybe_vector = alloc_with_size::<f64, System>(System, vector_size);

match maybe_vector {
  Ok(vec) => println!("Created a vec with size 10"),
  Err(error) => println!("Failed to create a vec, reason: {}", error)
}
```
As you could see, the maybe_vector has a ```Result<Vec<T>, falliable_alloc::AllocError> type```,
so now it's possible to handle a part of allocation errors.
