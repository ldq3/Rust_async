use std::collections::VecDeque;

use crate::future::Future;
use crate::runtime::waker::Waker;

pub struct Executor {
    all: Vec<Box<dyn Future<Output = ()>>>,
    num: i32,
    ready: VecDeque<i32>,
    finished: VecDeque<i32>,
}

pub struct Context {
    index: i32,
}

// 注册唤醒方法
impl Waker for Context {
    fn wake(&self) {
        
    }
}

impl Executor {
    fn spawn(&self, future: Box<dyn Future<Output = ()>>) {
        let id = self.num + 1;
        self.all.push(future);
        self.ready.push_back(id);
    }

    pub fn run(&self) {
        loop {
            let future_index = self.ready.pop_front().unwrap() as usize;
            let future = &self.all[future_index];
            
            let waker = Context {
                index: future_index
            };

            future.poll(waker);
        }
    }
}