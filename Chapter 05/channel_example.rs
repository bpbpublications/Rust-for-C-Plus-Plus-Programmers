// Goal is to write a concurrent program 
// that is able to send and then receive a message on a seperate thread
use std::thread;
use std::sync::mpsc::channel;
use std::io::{Result, stdin};

fn main() -> Result<()>{
    // create a new channel
    let (tx, rx) = channel();
    // create new string for input 
    let mut input = String::new();
    // create new thread to handle receiving message 
    let recv_handle = thread::spawn(move ||{
        // handle receive value using match 
        match rx.recv(){
            // if we get the message, print a received message 
            Ok(msg) => println!("Received message: {}", msg), 
            // if we get an error, print error to stderr
            Err(e) => eprintln!("{}", e.to_string())
        }
    });
    println!("Please enter a message to send: ");
    // get user input 
    stdin().read_line(&mut input)?;
    tx.send(input.trim().to_owned()).unwrap();
    //join receive handle to main thread 
    recv_handle.join().unwrap();
    Ok(())
}