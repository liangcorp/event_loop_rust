use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread;
use std::time::Duration;

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

pub struct EventLoop<T> {
    // head to the start of the task array
    task_array_head: TaskNode<T>,
    /*
        Mutex to enforce Mutual exclusion enqueue/dequeue
        operation in task array. Also used to update event loop
        attributes in mutual exclusive way
    */
    ev_loop_mutex: Arc<Mutex<u8>>,

    // state of event loop
    ev_loop_state: EVLoopState,

    // CV to suspended event loop thread
    ev_loop_cv: Arc<Condvar>,

    /*
        Current task which event loop thread is executing
        "None" if event loop is resting in peace
    */
    current_task: TaskNode<T>,
}

fn event_loop_get_next_task_to_run<T>(mut el: EventLoop<T>) -> Result<TaskNode<T>, String> {
    let task;
    if let Some(tah) = el.task_array_head {
        task = tah.clone();
    } else {
        return Err(format!(
            "Error <src/event_loop/mod.rs{}: unable to assign task from task_array_head",
            line!()
        ));
    }

    el.task_array_head = (*task).borrow().right.clone();

    task.borrow_mut().left = None;
    task.borrow_mut().right = None;

    Ok(Some(task))
}

fn event_loop_thread<T>(arg: Rc<RefCell<EventLoop<T>>>) {
    let el: Rc<RefCell<EventLoop<T>>> = arg;

    loop {
        thread::sleep(Duration::from_secs(2));
    }
}

pub fn event_loop_init<T>() -> Box<EventLoop<T>> {
    Box::new(EventLoop {
        task_array_head: None,
        ev_loop_mutex: Arc::new(Mutex::new(0)),
        ev_loop_state: EVLoopState::EVLoopIdle,
        ev_loop_cv: Arc::new(Condvar::new()),
        current_task: None,
    })
}

pub fn event_loop_run<T>(el: Rc<RefCell<EventLoop<T>>>) {
    thread::spawn (||{
    });
}
