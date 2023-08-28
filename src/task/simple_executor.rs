use super::Task;
use alloc::collections::VecDeque;
use core::task::{Waker, RawWaker};

fn dummy_raw_waker() -> RawWaker {
    todo!()
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker())}
}
pub struct SimpleExecutor {
    task_queue: VecDeque<Task>
}

impl SimpleExecutor {
    pub fn new() -> SimpleExecutor {
        SimpleExecutor { 
            task_queue: VecDeque::new() 
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task)
    }
}