
fn to_uint(i: i32) -> Option<u32>{
    // uint can only be positive integers
    if i >= 0{
        return Some(i as u32)
    } else {
        return None
    }
}


/*
    1. Unwrapping the error 
    let uint = to_uint(2).unwrap();

    2. Use match statement for the Option 
    let uint = match to_uint(2){
        Some(u) => u, 
        None => 0 as u32,
    };
*/
