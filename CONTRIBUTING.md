# fallible_alloc contribution guidelines

## Presequencies

It's assumed you have read [the rust book](https://doc.rust-lang.org/book/), [rust API guideline](https://rust-lang.github.io/api-guidelines/about.html) and are familiar with [the rust cookbook](https://rust-lang-nursery.github.io/rust-cookbook/).

## Introduction

At the moment we have an unstabilized allocations API in the std, so this is a temporary safe crate for a stable rust.

## Crate architecture short review

### Modules structure

The modules structure is the following:

![](https://github.com/zkud/fallible-alloc/blob/main/doc/images/modules.png)

As you could understand, lib is the root module of the project,
and uses modules defined for aspects of std lib, also there is a util module,
where reusable function should be placed.

### Directories structure

The directories structure is the following (some files have been ommitted):
```
fallible-alloc/
├── Cargo.toml
├── docs/
├── src/
|   └── lib.rs
|   └── box/
|   └── vec/ 
|   └── util/
├── tests/
└── target/
```

**Cargo.toml** -- standart Cargo's project's manifest. 
**docs/** -- a storage for documentation artifacts (f.i. diagrams). 
**src/lib.rs** -- crate's root file. 
**src/box** -- box's allocation functions. 
**src/vec** -- vec's allocation functions. 
**src/util** -- reusable components for the whole crate. 

### Code Style

The same code style is used everywhere as rustfmt utility's , also we use clippy to check warnings and code smells automatically.
Every commit is checked with this utilities.

## Contribution process

The gitflow approach is used in this project, so
your contribution should include the following steps:
1. Create a feature/fix branch, for example feature/map-allocations;
2. Create a pull request into the dev branch;
3. Resolve issues with merging;
4. Get an approval an get your branch merged into dev;
5. Get your branch merged into main and deployed to crates.io.
