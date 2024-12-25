# filz

An experimental Rust project exploring custom memory allocation strategies, built for learning and understanding low-level memory management concepts.

## Description

filz is a learning project that implements custom allocators to understand:

- How memory-mapped files work as a backing store for allocations
- Stack-based memory allocation strategies
- Rust's global allocator interface (`GlobalAlloc` trait)
- Safe memory management patterns in systems programming

This project is primarily educational and not intended for production use. Each implementation explores different memory management concepts.

## Memory Allocation Strategies

### Memory-Mapped File Allocator (`MAlloc`)

A simple allocator that uses memory-mapped files as backing storage:

- Maps files to memory for allocation
- Implements basic page tracking
- No deallocation support (educational simplification)
- Fixed file size with configurable capacity

```rust
use filz::mmap::MAlloc;

#[global_allocator]
static ALLOCATOR: MAlloc = MAlloc::new();

fn main() {
    // Allocations will be backed by a memory-mapped file
    let vec = vec![1, 2, 3, 4, 5];
}
```

### Stack Allocator (`StackAllocator`)

A basic stack-based memory allocator:

- Fixed-size memory buffer
- Simple bump allocation strategy
- No deallocation (educational simplification)
- Thread-safe allocation tracking

```rust
use filz::stack::StackAllocator;

#[global_allocator]
static ALLOCATOR: StackAllocator<4096> = StackAllocator::new();

fn main() {
    // Allocations will use the stack buffer
    let data = String::from("Hello, World!");
}
```

## Installation

If you want to experiment with this project, add it to your `Cargo.toml`:

```toml
[dependencies]
filz = "0.1.0"
```

## Learning Resources

This project explores several important concepts:

- **Memory Mapping**

  - Using `memmap2` for file-backed memory
  - Page alignment and memory layout
  - Virtual memory concepts

- **Stack Allocation**

  - Bump allocation strategy
  - Memory alignment requirements
  - Stack memory management

- **Global Allocation**
  - Implementing `GlobalAlloc` trait
  - Thread-safe memory allocation
  - Rust's memory allocator interface

## Project Structure

```
filz/
├── src/
│   ├── mmap.rs     # Memory-mapped file allocator
│   ├── stack.rs    # Stack-based allocator
│   └── lib.rs      # Library interface
└── memory/         # Directory for memory-mapped files
```

## Known Limitations

Since this is a learning project, it has several intentional limitations:

- No deallocation support in either allocator
- Fixed buffer sizes that must be set at compile time
- Memory leaks by design (educational simplification)
- No fragmentation handling
- Basic alignment calculations

## Development

To experiment with the allocators:

```bash
# Run with memory-mapped allocator
cargo run

# Switch allocators in main.rs and run with stack allocator
# Uncomment the StackAllocator and comment out MAlloc
cargo run
```

## Dependencies

- `memmap2`: Memory mapping functionality
- `libc`: System bindings for allocation

## Notes

This project is meant for learning about:

- Memory allocator implementation
- Rust's allocation interfaces
- Memory mapping concepts
- Low-level memory management

It is not intended for production use and deliberately omits many features that would be necessary in a real allocator.

## License

MIT License - See LICENSE for details

---

Built for 🦀 learning and experimentation
