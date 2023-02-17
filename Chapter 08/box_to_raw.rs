// box_to_raw.rs
fn to_raw(box_: Box<i32>) -> *mut i32{
    println!("Turning box of value {} into raw", *box_);
    Box::<i32>::into_raw(box_)
}

unsafe fn free_raw(raw: *mut i32){
    println!("Dropping raw pointer of value: {}", *raw);
    drop(raw)
}


fn main(){
    let box_: Box<i32> = Box::new(32);
    // consumes box and returns a mutable raw pointer to it 
    let raw: *mut i32 = to_raw(box_);
    unsafe{
        // change the value of the raw pointer 
        *raw = 87;
        free_raw(raw)
    }
}