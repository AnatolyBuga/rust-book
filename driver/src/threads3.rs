//! Mutex - mutual exclusion - similar to RefCell, but:
//! Allows only one thread to access data at any given time
//! When want access - signal to acquire mutex's lock
//! Lock keeps track of who has access to the data
//! mutex - guarding the data it holds via locking system
//!
//! Must attempt to acquire the lock before using data
//! when done with data - you must unlock the data
//! the call to lock of Mutex<T> returns a LockResult<smart pointer - MutexGuard>
//! MutexGuard implements Deref to point to inner data
//! Also has drop which releases data when MutexGuard out of scope
//!
//! Arc (Aromic rc) is similar to Rc, but SAFE in concurrent situations
//! Threadsafety(Arc) comes with a performane penalty
//!
//! Sync(can be ref from multiple threads) and Send(can be sent to a thread) Traits
//! T is Sync is &T is Send

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn mutexes() {
    let n = Arc::new(Mutex::new(0));
    // try use Rc here to move to multiple threads - But error - Rc cannot be sent between threads
    //trait Send is not implemented for Rc(count doesn't work across threads)
    //m: Mutex<i32>, NOT i32, have to lock to get access to i32
    let mut handles = vec![];

    //Notice, can't move n multiple times, so have to use multi ownership
    for _ in 0..10 {
        let n = Arc::clone(&n);
        let handle = thread::spawn(move || {
            let mut num = n.lock().unwrap();
            // !blocks current thread untill our time to use the lock
            //fails if another thread holding lock panics
            //num is like a mut ref to data inside
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("m = {:?}", n)
}

pub fn deadlock(){
    let m1 = Arc::new(Mutex::new(0));
    let m2 = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for i in 0..2 {
        let m1 = Arc::clone(&m1);
        let m2 = Arc::clone(&m2);
        handlers.push(thread::spawn(move || {
            if i == 0 {
                let g = m1.lock().unwrap();
                //m1 locked
                //sleep to let the other thread lock m2
                thread::sleep(Duration::from_millis(10));
                //wait for m2 to unlock
                let g = m2.lock().unwrap();//LOCKED HERE
            } else {
                //lock m2
                let g = m2.lock().unwrap();
                //wait for m1 to unlock
                let g = m1.lock().unwrap();//LOCKED HERE
            }
        }))
    }

    for h in handlers {
        h.join().unwrap();
    }
    println!("You will never get here")
}
