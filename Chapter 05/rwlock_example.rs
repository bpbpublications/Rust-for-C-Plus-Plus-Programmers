use std::fs::{read_dir, FileType, read_to_string};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread;
use std::io::Result;

// Represents all of our information needs 
#[derive(Debug, Clone)]
struct Info{
    type_: FileType, 
    content: String, 
    path: PathBuf,
}

impl Info{
    pub fn new(type_: FileType, content: String, path: PathBuf) -> Self{
        Self { type_, content, path }
    }
}
// create type alias for the shared state Info vector 
type Infos = Arc<RwLock<Vec<Info>>>;
// creates a new empty Infos 
fn new_infos() -> Infos{
    Arc::new(RwLock::new(Vec::new()))
}

fn main() -> Result<()>{
    // we will read a directory called important
    let dir = read_dir("important")?;
    // create a new shared state vector for infos 
    let infos = new_infos();
    // create a thread to read the directory 
    let t = thread::spawn(move ||{
        // clone infos into our thread 
        let i = infos.clone();
        // iterate through dir for each DirEntry 
        for d in dir{
            // unwrap the dir entry 
            let d = d.unwrap();
            // lock to write 
            let mut i = i.write().unwrap();
            // get the file type 
            let type_ = d.file_type().unwrap();
            // initiate the content 
            let mut content = String::new();
            // if content is directory, put "Directory"
            if type_.is_dir(){
                content.push_str("Directory")
            } else {
                // if file get the content from read_string()
                content = read_to_string(d.path()).unwrap()
            }
            // push the Info struct to i 
            i.push(Info::new(type_, content, d.path()))
        }
        // update our vector using read()
        let updated = i.read().unwrap();
        // iterate through updated using map to print each &Info
        let u = updated.iter().map(|x|{
            let mut type_ = String::new();
            // if type is directory, print Directory 
            if x.type_.is_dir(){
                type_.push_str("Directory")
            } else {
                // if type is file, print File
                type_.push_str("File")
            }
            // get the path as &str 
            let path = x.path.to_str().unwrap();
            println!("Path: {} | File type: {} \nContent: \n{}\n", 
            path, type_, x.content)
        }).collect::<()>();
        // return this collection of prints 
        u
    });
    // join the thread
    t.join().unwrap();
    // return Ok at the end 
    Ok(())
}