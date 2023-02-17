use std::io::{stdin, Write};
use std::fs::File;


fn main(){
    let mut path = String::new();
    // ask for path 
    println!("Please enter file path: ");
    // read input 
    stdin().read_line(&mut path).unwrap();
    // create new file from input 
    let mut file = File::create(&path.trim()).unwrap();
    // ask for content to write 
    let mut content = String::new();
    println!("Please enter content for file: ");
    stdin().read_line(&mut content).unwrap();
    file.write_all(&content.trim().as_bytes()).unwrap();
}