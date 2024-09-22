use core::alloc::GlobalAlloc;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct MAlloc {
    // page_no: AtomicUsize,
}

impl MAlloc {
    pub const fn new() -> Self {
        Self {
            // page_no: AtomicUsize::new(0),
        }
    }
}

impl Default for MAlloc {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl GlobalAlloc for MAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        // let page = self.page_no.fetch_add(1, Ordering::SeqCst);

        let size = layout.size();
        let align = layout.align();

        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("memory/page_1")
            .unwrap();

        file.set_len(size as u64).unwrap();

        let mut mmap = unsafe { memmap2::MmapMut::map_mut(&file).unwrap() };

        let output = mmap.as_mut_ptr();
        std::mem::forget(mmap);
        output
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // do nothing
    }
}
