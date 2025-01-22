use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Mutex, Condvar};
// use std::thread;

type TaskNode<T> = Option<Rc<RefCell<Task<T>>>>;

struct Task<T> {
    cbk: Box<dyn Fn()>,
    arg: T,
    left: TaskNode<T>,
    right: TaskNode<T>,
}

enum EVLoopState {
    EVLoopIdle,
    EVLoopBusy,
}

struct EventLoop<T> {
    // head to the start of the task array
    task_array_head: TaskNode<T>,
    /*
        Mutex to enforce Mutual exclusion enqueue/dequeue
        operation in task array. Also used to update event loop
        attributes in mutual exclusive way
    */
    ev_loop_mutex: Mutex<T>,

    // state of event loop
    ev_loop_state: EVLoopState,

    // CV to suspended event loop thread
    ev_loop_cv: Condvar,

    /*
        Current task which event loop thread is executing
        "None" if event loop is resting in peace
    */
    current_task: TaskNode<T>,
}

fn event_loop_get_next_task_to_run<T>(mut el: EventLoop<T>) -> TaskNode<T> {
    let task = el.task_array_head.unwrap();
    // if let Some(tah) = el.task_array_head {
    //     task = tah;
    // }
    el.task_array_head = Some(task.borrow_mut().right.clone()?);

    task.borrow_mut().left = None;
    task.borrow_mut().right = None;

    Some(task)
}
