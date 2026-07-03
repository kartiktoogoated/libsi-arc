use std::alloc::{Layout, alloc, dealloc};

fn main() {
    unsafe {
        let layout = Layout::from_size_align(32, 8).unwrap();
        let p = alloc(layout);

        *p = 10;
        // let value = *p;
        println!("{:?}", p);

        dealloc(p, layout);
        // println!("{}", value);
    }
}
