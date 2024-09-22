use core::alloc::GlobalAlloc;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct MAlloc<const N: u64 = 10000> {
    page_no: AtomicUsize,
}

impl<const N: u64> MAlloc<N> {
    pub const fn new() -> Self {
        Self {
            page_no: AtomicUsize::new(0),
        }
    }
}

impl<const N: u64> Default for MAlloc<N> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<const N: u64> GlobalAlloc for MAlloc<N> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        let page_no = self.page_no.fetch_add(size, Ordering::Relaxed);

        let aligned_page_no = (page_no + align + 1) & !(align + 1);

        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("memory/page_1")
            .unwrap();

        file.set_len(N).unwrap();

        let mut mmap = unsafe { memmap2::MmapMut::map_mut(&file).unwrap() };

        let output = mmap.as_mut_ptr().add(aligned_page_no);
        std::mem::forget(mmap);
        output
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // do nothing
    }
}
