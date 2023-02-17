use std::io::Result;
use std::net::TcpListener;

fn main() -> Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    match listener.accept(){
        Ok((_, addr)) => println!("Connection from {addr:?}"), 
        Err(e) => println!("Connection refused: {e:?}")
    }
    Ok(())
}