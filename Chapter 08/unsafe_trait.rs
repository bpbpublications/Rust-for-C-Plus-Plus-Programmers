// unsafe_trait.rs
use std::rc::Rc;
static mut COUNTER: u32 = 0;
#[derive(Copy, Clone)]
struct Empty;

unsafe trait TrustMe{
    fn return_box(self) -> Box<Self> where Self: Sized{
        Box::new(self)
    }
    unsafe fn return_ptr(self) -> (*mut Self, u32) where Self: Sized
    {
        let b = self.return_box();
        let ptr: *mut Self = Box::into_raw(b);
        COUNTER += 1;
        (ptr, COUNTER)
    }
}

fn free_empty(ptr: *mut Empty, id: u32){
    println!("Freeing empty struct with id: {}", id);
    drop(ptr)
}


unsafe impl TrustMe for Empty{}

fn main(){
    let empty  = Rc::new(Empty);
    let mut ptr_id = Vec::new();
    unsafe{
        for _ in 0..3{
            let empty = empty.clone();
            let (ptr, id) = empty.return_ptr();
            ptr_id.push((ptr, id));
        }
        for (ptr, id) in ptr_id{
            free_empty(ptr, id)
        }
    }   
}