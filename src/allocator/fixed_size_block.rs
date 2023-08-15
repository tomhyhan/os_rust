use alloc::alloc::Layout;
use core::ptr;

use super::Locked;
use alloc::alloc::GlobalAlloc;

unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllocator> {
    unsafe fn alloc(&self, layout:Layout) -> *mut u8 {
        let mut allocator = self.lock();
        match list_index(&layout) {
            Some(index) => {
                match allocator.list_heads[index].take() {
                    Some(node) => {
                        allocator.list_heads[index] = node.next.take();
                        node as *mut ListNode as *mut u8
                    }
                    None => {
                        // no block exist in list => allocate new block
                        let block_size = BLOCK_SIZE[index];
                        // only works if all block sizes are a power of 2
                        let block_align = block_size;
                        let layout = Layout::from_size_align(block_size, block_align).unwrap();
                        allocator.fallback_alloc(layout)
                    }
                }
            }
            None => allocator.fallback_alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}


struct ListNode {
    next: Option<&'static mut ListNode>
}

// The block sized to use
// 
// The sizes must each be power of 2 becuase they are also used as the block alignment (alignment must be always powers of 2)
const BLOCK_SIZE: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

pub struct FixedSizeBlockAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZE.len()],
    fallback_allocator: linked_list_allocator::Heap
}

impl FixedSizeBlockAllocator {
    // creates an empty FixedSizeBlockAllocator.
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        FixedSizeBlockAllocator {
            list_heads: [EMPTY; BLOCK_SIZE.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    // init the allocator with the given heap bounds
    // 
    // This function is unsafe becuase the caller must guarantee that the given heap bounds are valid and that the heap is unused. This method must be called only once
    pub unsafe fn init(&mut self, heap_start: usize, heap_size:usize) {
        self.fallback_allocator.init(heap_start, heap_size)
    }

    // Allocates using the fallback Allocator.
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}

// Choose an appropriate block size for the given layout
// 
// Returns an index into the 'BlOCK_SIZES' array.
fn list_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZE.iter().position(|&s| s >= required_block_size)
}