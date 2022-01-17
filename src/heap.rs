use core::alloc::GlobalAlloc;

pub struct MosAllocator;

extern "C" {
    fn malloc(n: usize) -> *mut u8;
    fn free(ptr: *mut u8);
    fn __heap_bytes_free() -> usize;
    fn __heap_bytes_used() -> usize;
    fn __set_heap_limit(limit: usize);
    fn __heap_limit() -> usize;
}

pub fn bytes_free() -> usize {
    unsafe { __heap_bytes_free() }
}
pub fn bytes_used() -> usize {
    unsafe { __heap_bytes_used() }
}

pub fn set_limit(limit: usize) {
    unsafe { __set_heap_limit(limit) }
}
pub fn get_limit() -> usize {
    unsafe { __heap_limit() }
}

unsafe impl GlobalAlloc for MosAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        free(ptr);
    }
}

#[global_allocator]
static ALLOCATOR: MosAllocator = MosAllocator;
