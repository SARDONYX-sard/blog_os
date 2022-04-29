use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
};

struct ListNode {
    next: Option<&'static mut ListNode>,
}

/// The block sizes to use.
///
/// The sizes must each be power of 2 because they are also used as
/// the block alignment (alignments must be always powers of 2).
const BLOCK_SIZE: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

pub struct FixedSizeBlockAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZE.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

impl FixedSizeBlockAllocator {
    // Creates an empty FixedSizeBlockAllocator.
    pub const fn new() -> Self {
        const Empty: Option<&'static mut ListNode> = None;
        FixedSizeBlockAllocator {
            list_heads: [Empty; BLOCK_SIZE.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    /// Initializes the allocator with the given heap bounds.
    ///
    /// This function is unsafe because the caller must guarantee that the given
    /// heap bounds are valid and that the heap is unused. This method must be
    /// called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_allocator.init(heap_start, heap_size);
    }
}

impl FixedSizeBlockAllocator {
    /// Allocates using the fallback allocator.
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}

/// Choose an appropriate the block size for the given Layout.
/// Returns None if the layout is not suitable for a FixedSizeBlockAllocator.
fn list_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    //  to use the returned index as an index into the list_heads array
    BLOCK_SIZE.iter().position(|&s| s >= required_block_size)
}
