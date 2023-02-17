use std::io::Error;
use std::fs;

fn main() -> Result<(), Error>{
    let source = fs::read_to_string("foo.txt");
/*

How do we handle source?

1. Unwrap the error, and panic if error occurs 
let source = fs::read_to_string("foo.txt").unwrap();

2. Expect the error, and print a message if panics 
let source = fs::read_to_string("foo.txt").expect("Couldn't read file");

3. Match the Ok and Err values 
let source = match fs::read_to_string("foo.txt"){
    Ok(s) => s, 
    Err(e) => {
        return Err(Error::from(e))
    }
};

4. Use the ? operator since read_to_string returns the same std::io::Error as our Result 
let source = fs::read_to_string("foo.txt")?;


*/

    Ok(())
}