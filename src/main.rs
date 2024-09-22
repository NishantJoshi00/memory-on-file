use core::hint::black_box;

use filz::mmap::MAlloc;
use filz::stack::StackAllocator;

#[global_allocator]
static ALLOCATOR: MAlloc = MAlloc::new();

// #[global_allocator]
// static ALLOCATOR: StackAllocator<4096> = StackAllocator::new();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![105, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];

    black_box(data);

    Ok(())
}
