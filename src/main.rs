use std::{rc::Rc, cell::RefCell};
use std::sync::mpsc;
use event_loop_rust::event_loop::*;

fn main() {
    let el: EventLoop<i32> = event_loop_init::<i32>();
    event_loop_run(Rc::new(RefCell::new(el)));
    let (tx, rx) = mpsc::channel::<i32>();
}
