use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use alloc::collections::VecDeque;

use super::Task;

pub struct SimpleTaskExecutor {
    task_queue: VecDeque<Task>,
}

impl SimpleTaskExecutor {
    pub fn new() -> Self {
        SimpleTaskExecutor {
            task_queue: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task);
    }
}

impl SimpleTaskExecutor {
    pub fn run(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Ready(_) => {}
                Poll::Pending => self.task_queue.push_back(task),
            }
        }
    }
}

fn dummy_raw_worker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_worker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const (), vtable)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_worker()) }
}
