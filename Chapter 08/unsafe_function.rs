// unsafe_function.rs
static mut ACTIVE: bool = false;

// the user should know this operation is unsafe 
unsafe fn change_active(active: bool){
    ACTIVE = active
}

fn main(){
    unsafe{
    // do something 
    println!("Currently task is {}", ACTIVE);
    // now we need ACTIVE to be true 
    change_active(true);
    println!("Activity of task is now {}", ACTIVE);
    // do something more 
    // reset ACTIVE to false
    change_active(false);
    println!("Task is reset to {}", ACTIVE);
    }
}