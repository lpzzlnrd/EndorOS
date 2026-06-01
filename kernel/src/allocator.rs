use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Size of the static heap in bytes (2 MiB for the demo kernel).
const HEAP_SIZE: usize = 2 * 1024 * 1024;

/// Static backing store for the bump allocator.
/// Aligned to 16 bytes so all common types are satisfied.
#[repr(align(16))]
struct HeapStorage([u8; HEAP_SIZE]);

static HEAP: HeapStorage = HeapStorage([0u8; HEAP_SIZE]);

/// Bump allocator: allocates memory by moving a pointer forward.
/// Memory is never freed — this is intentional for a simple OS demo.
pub struct BumpAllocator {
    /// Byte offset of the next free slot inside HEAP.
    next: AtomicUsize,
    /// Base address of the heap (set lazily on first use).
    base: UnsafeCell<usize>,
}

/// SAFETY: The BumpAllocator uses an atomic counter for the bump pointer, so
/// concurrent calls are safe. The base pointer is written once at startup.
unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            next: AtomicUsize::new(0),
            base: UnsafeCell::new(0),
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Lazily store the heap base address.
        let base = HEAP.0.as_ptr() as usize;

        let align = layout.align();
        let size = layout.size();

        loop {
            let current = self.next.load(Ordering::Relaxed);
            // Align the bump pointer.
            let aligned = (base + current + align - 1) & !(align - 1);
            let new_next = aligned - base + size;

            if new_next > HEAP_SIZE {
                // Out of heap memory — return null.
                return core::ptr::null_mut();
            }

            // CAS to claim the region atomically.
            match self.next.compare_exchange(
                current,
                new_next,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return aligned as *mut u8,
                Err(_) => continue, // Retry on contention.
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocators do not free memory.
    }
}

#[global_allocator]
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new();
