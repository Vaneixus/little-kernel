use good_memory_allocator::SpinLockedAllocator;
#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

pub const HEAP_START: usize = 0x2222_2222;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_heap() {
    unsafe {
        ALLOCATOR.init(HEAP_START, HEAP_SIZE);
    }
}