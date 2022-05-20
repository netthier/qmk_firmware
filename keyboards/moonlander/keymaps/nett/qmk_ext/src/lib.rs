#![no_std]
#![feature(alloc_c_string)]
#![feature(let_chains)]
#[macro_use]
extern crate alloc;

#[global_allocator]
static ALLOCATOR: qmk::malloc::MallocFreeAlloc = qmk::malloc::MallocFreeAlloc;

pub mod keymap;
