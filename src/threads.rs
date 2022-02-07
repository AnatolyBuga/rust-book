use std::thread;
use std::time::Duration;

/// Rust uses 1:1 model, ie 1 operating sys thread per one language thread
/// !No guarantee of order
pub fn thread() {
    let v = vec![1,2,3];
    let handle = thread::spawn( move || {
        for j in 1..10{
            println!("Number {} from thread spawn", j);
            thread::sleep(Duration::from_millis(1)); //pauses thread, allows other thread to run
            println!("{:?}", v) //println only needs reference so closure tries to BORROW v
            //but Rust doesn't know how long will thread live and hence if ref to v will be valid
            //eg if dropped, see below
            //so, force MOVE to force ownership
        }
    });

    //drop(v);

    for i in 1..5 {
        println!("From Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //wait for all to finish
    handle.join().unwrap();
}