use std::io::{stdin, Result};
use std::sync::{Arc, Mutex};
use std::thread;
pub type Students = Arc<Mutex<Vec<u32>>>;

fn new_empty_students() -> Students {
    Arc::new(Mutex::new(Vec::new()))
}

fn get_grade(student_num: u32) -> u32 {
    // get user input of grade
    println!("Please enter grade for student {}: ", student_num);
    // create empty string for input 
    let mut input = String::new();
    // read stdin for input 
    stdin().read_line(&mut input).unwrap();
    // trim then parse the string as u32 
    input.trim().parse().unwrap()
}

fn main() -> Result<()> {
    let students = new_empty_students();
    let mut join_handles = Vec::new();

    // get number of students
    println!("Please enter number of students: ");
    let mut num_input = String::new();
    stdin().read_line(&mut num_input)?;
    // parse input into u32
    let num_stu: u32 = num_input.trim().parse().unwrap();
    // loop to get each student's grades
    for i in 0..num_stu {
        // get shared access to students
        let s = students.clone();
        // create a thread to enter student's grade
        let grade = get_grade(i + 1);
        let t = thread::spawn( move || {
            let mut s = s.lock().unwrap();
            s.push(grade)
        });
        join_handles.push(t);
    }

    let t = thread::spawn(move ||{
       let s = students.lock().unwrap();
        for i in 0..s.len(){
            println!("Student {} => Grade: {}", i+1, s[i])
        }
    });

    join_handles.push(t);

    for jh in join_handles {
        jh.join().unwrap();
    }
    Ok(())
}
