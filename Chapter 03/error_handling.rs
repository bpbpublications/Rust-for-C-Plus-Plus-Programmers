// Rust Example using the Result type 
// Our program will attempt to login with a password 
fn login(password: u32) -> Result<String, String>{
    let actual: u32 = 97979;
    if actual == password{
       return Ok(format!("Logged in successfully!"));
    } else {
        return Err(format!("Login denied: {} is wrong!", password));
    }
}

fn main(){
    match login(8888){
        Ok(o) => println!("{}", o), 
        Err(e) => println!("{}", e)
    }
}

// Output: 
// Login denied: 8888 is wrong!

