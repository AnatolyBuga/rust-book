use crate::smart_pointers3::{LimitTracker, Messenger};
use super::smart_pointers3;
use std::cell::RefCell;

struct MockMessenger {
    //sent_messages: Vec<String>,
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger{
    fn new() -> MockMessenger{
        MockMessenger{
            //sent_messages: vec![],
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        //can't use &mut self, because then signature of send won't match signature of send in
        //Messenger trait def
        //IF NOT REFCELL - cannot modify self, because immut &self
        self.sent_messages.borrow_mut().push(String::from(message));
        //NOTICE with REFCELL we can BORROW MUT and then push an immut ref
    }
}

#[test]
fn it_sends_90_message() {
    let mM = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mM, 100);
    limit_tracker.set_value(91);
    //assert_eq!(mM.sent_messages.len(), 1);
    assert_eq!(mM.sent_messages.borrow().len(), 1);
    //have to call borrow to get immut ref to the vector
}