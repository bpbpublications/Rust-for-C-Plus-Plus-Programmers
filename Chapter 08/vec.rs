use std::ptr::{NonNull, write, read, self};
use std::marker::PhantomData;
use std::mem;
use std::alloc::{self, Layout};
use std::ops::{Deref, DerefMut};


struct RawVec<T>{
    ptr: NonNull<T>, 
    cap: usize, 
    _marker: PhantomData<T>
}
unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T>{
    fn new() -> Self{
        assert!(mem::size_of::<T>() != 0, "TODO: implement ZST support");
        RawVec { ptr: NonNull::dangling(), cap: 0, _marker: PhantomData }
    }
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow because we ensure self.cap <= isize::MAX.
            let new_cap = 2 * self.cap;

            // Layout::array checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

pub struct Vec<T> {
    // raw vector 
    buf: RawVec<T>, 
    // length of vector 
    len: usize
}

// impl send and sync for vector
unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Vec<T> {
    fn ptr(&self) -> *mut T{
        self.buf.ptr.as_ptr()
    }
    fn cap(&self) -> usize{
        self.buf.cap
    }
    pub fn new() -> Self {
        Vec { buf: RawVec::new(), len: 0 }
    }
    pub fn len(&self) -> usize{
        self.len
    }
    pub fn push(&mut self, elem: T){
        // if length is same as capacity, we will grow vector's capacity 
        if self.len == self.cap() {self.buf.grow()}
        unsafe{
            write(self.ptr().add(self.len), elem);
        }
        // increase length by 1 
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<T>{
        if self.len == 0{
            None 
        } else {
            self.len -= 1;
            unsafe{
                Some(read(self.ptr().add(self.len)))
            }
        }
    }
    pub fn insert(&mut self, index: usize, elem: T){
        // <= is valid because elements can be inserted after everything 
        assert!(index <= self.len, "index is out of bounds");
        if self.cap() == self.len {self.buf.grow()}

        unsafe{
            // ptr::copy(src, dest, len) "copy from src to dest len elems"
            ptr::copy(self.ptr().add(index),
                self.ptr().add(index + 1), 
                self.len - index
            );
            write(self.ptr().add(index), elem);
            self.len += 1;
        }
    }
    pub fn remove(&mut self, index: usize) -> T{
        assert!(index < self.len, "index is out of bounds");
        unsafe{
            self.len -= 1;
            let result = read(self.ptr().add(index));
            ptr::copy(self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index
            );
            result
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
        // deallocation is handled by RawVec
    }
}


// deref vec into &[T] and &mut [T]
impl<T> Deref for Vec<T>{
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe{
            std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

impl <T> DerefMut for Vec<T>{
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe{
            std::slice::from_raw_parts_mut(self.ptr(), self.len)
        }
    }
}

// so we can iterate 
pub struct IntoIter<T>{
    _buf: RawVec<T>, 
    start: *const T, 
    end: *const T
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        unsafe {
            // need to use ptr::read to unsafely move the buf out since it's
            // not Copy, and Vec implements Drop (so we can't destructure it).
            let buf = ptr::read(&self.buf);
            let len = self.len;
            mem::forget(self);

            IntoIter {
                start: buf.ptr.as_ptr(),
                end: if buf.cap == 0 {
                    // can't offset off of a pointer unless it's part of an allocation
                    buf.ptr.as_ptr()
                } else {
                    buf.ptr.as_ptr().add(len)
                },
                _buf: buf,
            }
        }
    }
}


// iterating forward 
impl<T> Iterator for IntoIter<T>{
    type Item = T; 
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end{
            None
        } else {
            unsafe{
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

// iterating backwards 
impl<T> DoubleEndedIterator for IntoIter<T>{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end{
            None
        } else {
            unsafe{
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

// since IntoIter takes ownership, we want to be able to drop it 
impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // only need to ensure all our elements are read;
        // buffer will clean itself up afterwards.
        for _ in &mut *self {}
    }
}

fn main(){
    /* 
    let mut vec = Vec::new();
    for i in 1..=25{
        vec.push(i);
    }
    vec.insert(1, 5);
    for _ in 0..vec.len(){
        println!("{:?}", vec.pop())
    }
    */
    /*
    let mut vec = Vec::new();
    for i in 1..20{
        vec.push(i)
    }
    let slice = &mut *vec;
    slice[8] = 78;
    println!("{:?}", slice)
    */
    let mut vec = Vec::new();
    for i in 1..=5{
        vec.push(i)
    }
    for j in vec.into_iter(){
        println!("{}", j * 2)
    }
}