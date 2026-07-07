use libc::{c_void, free, malloc, realloc};
use std::fmt;
use std::ptr::{NonNull, copy_nonoverlapping};
use std::slice;

struct TinyBuf {
    ptr: NonNull<u8>,
    len: usize,
    cap: usize,
}

impl TinyBuf {
    fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "capacity must be > 0");

        unsafe {
            let raw = malloc(cap) as *mut u8;
            let ptr = NonNull::new(raw).expect("alloc failed");
            Self { ptr, len: 0, cap }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    pub fn reserve(&mut self, additional: usize) {
        let needed = self.len() + additional;
        if needed <= self.cap() {
            return;
        }

        let mut new_cap = self.cap().max(1);
        while new_cap < needed {
            new_cap *= 2;
        }

        unsafe {
            let raw = realloc(self.ptr.as_ptr() as *mut c_void, new_cap) as *mut u8;
            self.ptr = NonNull::new(raw).expect("realloc failed");
            self.cap = new_cap;
        }
    }

    fn push(&mut self, byte: u8) {
        self.reserve(1);
        unsafe {
            *self.ptr.as_ptr().add(self.len) = byte;
        }
        self.len += 1;
    }

    fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.reserve(bytes.len());
        unsafe {
            let dst = self.ptr.as_ptr().add(self.len);
            copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
        }
        self.len += bytes.len();
    }

    fn pop(&mut self) -> Option<u8> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        unsafe { Some(*self.ptr.as_ptr().add(self.len)) }
    }

    fn shrink_to_fit(&mut self) {
        let target = self.len.max(1);
        if target == self.cap() {
            return;
        }

        unsafe {
            let raw = realloc(self.ptr.as_ptr() as *mut c_void, target) as *mut u8;
            self.ptr = NonNull::new(raw).expect("realloc failed");
            self.cap = target;
        }
    }
}

impl Drop for TinyBuf {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr.as_ptr() as *mut c_void);
        }
    }
}

impl fmt::Debug for TinyBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TinyBuf")
            .field("ptr", &self.ptr)
            .field("len", &self.len)
            .field("cap", &self.cap)
            .field("bytes", &self.as_slice())
            .finish()
    }
}

fn main() {
    let mut buf = TinyBuf::with_capacity(8);

    buf.push(b'h');
    buf.push(b'i');
    buf.push(b' ');
    buf.extend_from_slice(b"there");
    buf.push(b'!');

    println!("buf = {:?}", buf);
    println!("len = {}", buf.len());
    println!("cap = {}", buf.cap());
    println!("text = {}", String::from_utf8_lossy(buf.as_slice()));

    if let Some(last) = buf.pop() {
        println!("popped = {} ({})", last, last as char);
    }

    println!("after pop = {:?}", buf);

    buf.shrink_to_fit();
    println!("after shrink = {:?}", buf);
}
