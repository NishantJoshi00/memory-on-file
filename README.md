# filz

`filz` is a Rust library that provides custom memory allocation strategies, including memory-mapped file allocation and stack allocation. This is a toy project to learn more about Rust and memory allocation.

## Features

- Memory-mapped file allocation (`MAlloc`)
- Stack allocation (`StackAllocator`)
- Implements the `GlobalAlloc` trait for custom global allocation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
filz = "0.1.0"
```

## Usage

### Memory-mapped File Allocation

To use the memory-mapped file allocator:

```rust
use filz::mmap::MAlloc;

#[global_allocator]
static ALLOCATOR: MAlloc = MAlloc::new();

fn main() {
    // Your code here
}
```

### Stack Allocation

To use the stack allocator:

```rust
use filz::stack::StackAllocator;

#[global_allocator]
static ALLOCATOR: StackAllocator<4096> = StackAllocator::new();

fn main() {
    // Your code here
}
```

## Example

```rust
use core::hint::black_box;
use filz::mmap::MAlloc;

#[global_allocator]
static ALLOCATOR: MAlloc = MAlloc::new();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![105, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
    black_box(data);
    Ok(())
}
```

## Project Structure

- `src/lib.rs`: Main library file
- `src/mmap.rs`: Implementation of memory-mapped file allocation
- `src/stack.rs`: Implementation of stack allocation

## Dependencies

- `libc`: "0.2.158"
- `memmap2`: "0.9.5"
