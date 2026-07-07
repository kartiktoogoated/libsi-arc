use libc::{c_void, malloc, realloc};
use std::ptr::NonNull;

fn main() {
    unsafe {
        let raw = malloc(8) as *mut u8;
        let mut ptr = NonNull::new(raw).expect("alloc failed");

        let old = ptr.as_ptr();

        let new_raw = realloc(ptr.as_ptr() as *mut c_void, 16) as *mut u8;
        ptr = NonNull::new(new_raw).expect("realloc failed");

        // use the new pointer
        *ptr.as_ptr() = 42;

        //old is now stale
        println!("old add = {:?}", old);
        println!("new add = {:?}", new_raw);
    }
}
