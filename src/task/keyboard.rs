use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use crate::println;

/// You can also use the lazy_static macro. However,
/// the OnceCell type has the advantage of preventing the interrupt handler
/// from allocating the heap by ensuring that
/// initialization does not take place within the interrupt handler.
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

// Because this interrupt handler should not allocate heap memory.
// It must not be callable from main.rs, so `pub(crate)` is used.
// Available only from lib.rs.
/// Called from the keyboard interrupt handlers.
///
/// Please don't block or allocate processing.
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        Self { _private: () }
    }
}
