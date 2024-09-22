use core::alloc::GlobalAlloc;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct MAlloc<const N: u64 = 10000> {
    page_no: AtomicUsize,
    mmap_ptr: AtomicUsize,
}

impl<const N: u64> MAlloc<N> {
    const FILE_NAME: &'static str = "memory/page_1";

    pub const fn new() -> Self {
        Self {
            page_no: AtomicUsize::new(0),
            mmap_ptr: AtomicUsize::new(0),
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

        let mmap_ptr = self.mmap_ptr.load(Ordering::Relaxed);

        let output = if mmap_ptr == 0 {
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(Self::FILE_NAME)
                .unwrap();

            if file.set_len(N).is_err() {
                return std::ptr::null_mut();
            }

            let mmap = memmap2::MmapMut::map_mut(&file);

            let mut mmap = match mmap {
                Ok(mmap) => mmap,
                Err(_) => return std::ptr::null_mut(),
            };

            let output = mmap.as_mut_ptr();

            self.mmap_ptr.store(output as usize, Ordering::Relaxed);

            std::mem::forget(mmap);
            output
        } else {
            mmap_ptr as *mut u8
        };

        output.add(aligned_page_no)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // do nothing
    }
}
