// raw_pointer.rs
fn main(){
    let x: i32 = 32;
    let y = &x as *const i32;
    unsafe{
    println!("This is y: {}", *y);
    }
}