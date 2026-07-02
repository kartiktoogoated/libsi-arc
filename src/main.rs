use libc::{free, malloc};
use std::ptr::NonNull;

fn alloc_bytes(n: usize) -> Option<NonNull<u8>> {
    unsafe { NonNull::new(malloc(n) as *mut u8) }
}

fn main() {
    let ptr = alloc_bytes(32).expect("alloc failed");
    unsafe {
        for i in 0..33 {
            *ptr.as_ptr().add(i) = 0x99;
        }
        free(ptr.as_ptr() as *mut libc::c_void);
    }
}
