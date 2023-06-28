//! Channel: transmitter - reciever; send data - recieve

///mpsc - multiple producer single consumer
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn thread() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx3 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("From Spawn1"),
            String::from("With Love"),
            String::from("Anatoly"),
        String::from("Bugakov")];
        for val in vals {
            let tx2 = tx1.clone();
            thread::spawn(move || { //spawning thread from within a thread
            tx2.send(val).unwrap(); //notice spawn owns tx
            thread::sleep(Duration::from_secs(1))
            });
            //send returns Result<T,E>. If recieving end dropped, unwrap will panic
            //But should be handled properly
            //can't use val after it's been sent
            thread::sleep(Duration::from_secs(1))
        }});
    thread::spawn(move || {
        let vals = vec![
            String::from("From Spawn2"),
            String::from("Me too"),
            String::from("You too"),
        String::from("Love you")];
        for val in vals {
            tx3.send(val).unwrap(); //notice spawn owns tx
            //send returns Result<T,E>. If recieving end dropped, unwrap will panic
            //But should be handled properly
            //can't use val after it's been sent
            thread::sleep(Duration::from_secs(1))
        }});

    let recieved_first = rx.recv().unwrap(); //BLOCKs main thread untill sent. Once sent, recv will
    //return Result<T, E>
    //When sending end of the channel closes , recv will return error
    //try_recv doesn't block but will return Result<T,E> immidiately (Ok if message sent, otherwise Err)
    println!("Recieved First: {}", recieved_first);
    let recieved_second = rx.recv().unwrap();
    println!("Recieved Second: {}", recieved_second);
    //rx seems to be a generator where recv acts as next

    for recieved_all_the_rest in rx {
        println!("Got: {}", recieved_all_the_rest)
    }
}