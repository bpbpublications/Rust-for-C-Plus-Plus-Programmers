use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{self, AtomicUsize, Ordering};
// Atomic reference counter
pub struct Arc<T> {
    // pointer of inner reference counting
    ptr: NonNull<ArcInner<T>>,
    // marker to tell drop checker we have ownership over ArcInner
    phantom: PhantomData<ArcInner<T>>,
}

pub struct ArcInner<T> {
    // reference counting
    rc: AtomicUsize,
    // inner data
    data: T,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        // start reference counting at 1
        // create a new boxed value which we can use for NonNull
        let boxed = Box::new(ArcInner {
            rc: AtomicUsize::new(1),
            data,
        });
        Arc {
            ptr: NonNull::new(Box::into_raw(boxed)).unwrap(),
            phantom: PhantomData,
        }
    }
    pub fn count(&self) -> usize {
        // get inner value
        let inner = unsafe { self.ptr.as_ref() };
        inner.rc.load(Ordering::Acquire)
    }
}

// implement send and sync for Arc, making sure T has sync and send
unsafe impl<T: Sync + Send> Send for Arc<T> {}
unsafe impl<T: Sync + Send> Sync for Arc<T> {}

// dereferencing Arc<T> to get inner value
impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // get inner by using NonNull::as_ref
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}

// to be able to clone Arc needs the following
// 1. increment the reference count
// 2. create a new instance of Arc from inner pointer

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Arc<T> {
        // get inner arc
        let inner = unsafe { self.ptr.as_ref() };
        //using relaxed ordering to update reference count
        let old_rc = inner.rc.fetch_add(1, Ordering::Relaxed);
        // if reference count is overflowing abort process
        if old_rc >= isize::MAX as usize {
            std::process::abort();
        }
        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

// to be able to drop Arc needs the following
// 1. decrement the reference count
// 2. if there is old reference is 1:
//      1. atomically fence the data to prevent reordering of the use and deletion of data
//      2. drop the inner data
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // get inner value
        let inner = unsafe { self.ptr.as_ref() };
        if inner.rc.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }
        // fence the data to prevent reordering of the use and
        // deletion of the inner data
        atomic::fence(Ordering::Acquire);
        unsafe { Box::from_raw(self.ptr.as_ptr()) };
    }
}

fn main() {
    let arc: Arc<i32> = Arc::new(32);
    let mut threads = Vec::new();
    println!("Owners before first closure: {} with value of {}", arc.count(), *arc);
    {
        let arc2 = arc.clone();
        println!("Owners in first closure: {} with value of {}", arc.count(), *arc2 *2);
    }
    for i in 0..10{
        let new_arc = arc.clone();
        let t = std::thread::spawn(move ||{
            println!("Number of owners: {} with value of {}", new_arc.count(), *new_arc * i)
        });
        threads.push(t);
    }
    for child in threads{
        child.join().unwrap();
    }
    println!("Owners in the end: {} with value of {}", arc.count(), *arc)
}
