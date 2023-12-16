use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_heap() {
    let mut heap_start: u8  = 0x_ff;

    unsafe {
        ALLOCATOR.lock().init(&mut heap_start as *mut u8, HEAP_SIZE);
    }
}