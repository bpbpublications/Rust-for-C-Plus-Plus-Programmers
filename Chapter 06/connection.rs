use std::net::TcpStream;


fn main(){
    if let Ok(_stream) = TcpStream::connect("127.0.0.1:8080"){
        println!("Connected to listener!")
    } else {
        println!("Failed to connect to listener...")
    }
}