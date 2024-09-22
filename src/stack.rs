use core::alloc::GlobalAlloc;
use core::sync::atomic::AtomicUsize;

pub struct StackAllocator<const N: usize> {
    stack: [u8; N],
    allocated: AtomicUsize,
}

impl<const N: usize> StackAllocator<N> {
    pub const fn new() -> Self {
        Self {
            stack: [0; N],
            allocated: AtomicUsize::new(0),
        }
    }
}

impl<const N: usize> Default for StackAllocator<N> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<const N: usize> GlobalAlloc for StackAllocator<N> {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        let allocated = self
            .allocated
            .fetch_add(size, std::sync::atomic::Ordering::SeqCst);

        let aligned = (allocated + align - 1) & !(align - 1);

        if allocated + layout.size() > N {
            return std::ptr::null_mut();
        }
        self.stack.as_ptr().add(aligned) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: std::alloc::Layout) {
        // not implemented
    }
}
