use std::{
    os::fd::RawFd,
    collections::HashMap,
};

use epoll::{
    Events,
    Event,
    ControlOptions::EPOLL_CTL_ADD,
};

use crate::runtime::waker::Waker;

pub struct Reactor {
    epoll: RawFd,
    pending: HashMap<RawFd, Box<dyn Waker>>,
}

impl Reactor {
    pub fn new() -> Reactor {
        Reactor {
            epoll: epoll::create(false).unwrap(),
            pending: HashMap::new(),
        }
    }

    pub fn add(&mut self, fd: RawFd, waker: Box<dyn Waker>) {
        let event = epoll::Event::new(Events::EPOLLIN | Events::EPOLLOUT, fd as u64);
        epoll::ctl(self.epoll, EPOLL_CTL_ADD, fd, event).unwrap();
        self.pending.borrow_mut().insert(fd, waker);
    }

    pub fn wait(&mut self) {
        let mut events = [Event::new(Events::empty(), 0); 1024];
        let num_events = epoll::wait(self.epoll, -1, &mut events).unwrap();

        for event in &events[..num_events] {
            let fd  = event.data as i32;

            if let Some(waker) = self.pending.borrow_mut().get(&fd) {
                waker.wake();
            }
        }
    }
}