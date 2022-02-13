//! raw pointers: *const T and *mut T, can only be derefed in unsafe block
//! Constants are better than statics. Const is immut
//! 
use std::slice;
static RISK_WEIGHT: f64 = 0.15;
const XXX: u32 = 180;

pub fn _unsafe() {
    let a = 5;
    let r = &a as *const i32;
    unsafe{ 
        println!("{}", *r);
        // Unsafe fn can only be called from unsafe blocks
        danger()
    }

    let mut v: Vec<u32> = (1..=6).collect();
    let r = &mut v[..];
    let(a, b) = r.split_at_mut(3);
    
}

unsafe fn danger() {}

fn split_at_mut(_slice: &mut [u32], mid: usize) -> (&mut [u32], &mut [u32]) {
    let len = _slice.len();
    let ptr = _slice.as_mut_ptr();
    //we are MUT borrowing different parts of slice
    //but borrow checker doesn't understand, so can't do this
    //(&mut slice[..mid], &mut slice[mid..])
    assert!(mid<=len);
    unsafe {
        (
            //unsafe because takes raw pointer ptr and trusts it to be valid
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid),len - mid),
        )
    }
}

/// Run a C function
extern "C" {
    fn abs(input: i32) -> i32;
}

/// Run this Rust function in C
#[no_mangle]
pub extern "C" fn call_from_c(){
    println!("Just called a Rust function")
}