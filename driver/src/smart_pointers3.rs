//! Interior mutability - allows you to mutate data even when there are immut ref to data
//! Normally disallowed by borrowing rules, so uses unsafe.
//! RefCell - single ownership. Borrowing rules enforced AT RUNTIME (not compile time). Only single threaded.
//! allows mut and immut borrows checked at runtime
//! .borrow() and .borrow_mut() return Ref<T> and RefMut<T>
//! RefCell keeps track of how many Ref<T> and RefMut<T> smart pointer are currently active
//! Can have many imut or one mut at time - if violate we get RUNTIME error

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        //only one pointer here so 'a not required in fun new def
        Self{
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent_of_max = self.value as f64 / self.max as f64;
        if percent_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!")
        } else if percent_of_max >= 0.9 {
            self.messenger.send("Urgent! You've used 90% of your quota!")
        } else {
            self.messenger.send("You are under 90%")
        }
    }
}
