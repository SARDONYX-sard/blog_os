use super::align_up;
use core::mem;

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new() -> Self {
        ListNode {
            size: 0,
            next: None,
        }
    }

    fn start_addr(&self) -> usize {
        self as *const ListNode as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}
