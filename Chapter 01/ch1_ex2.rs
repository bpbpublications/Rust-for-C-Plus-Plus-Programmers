use std::collections::HashMap;
use std::io::stdin;
fn main() {
    let mut input = String::new();
    // Ask and get number of students
    println!("Please enter the number of students: ");
    stdin().read_line(&mut input).unwrap();
    let num_of_stu: u8 = input.trim().parse().unwrap();
    // Key is student name
    // Value is student's grade
    let mut students: HashMap<String, f32> = HashMap::new();
    for i in 1..=num_of_stu{
        enter_student(&mut students, i);
    }
    // At the end we will print the hashmap 
    println!("List of Students: ");
    for (name, grade) in &students{
        println!("{name}: {grade}")
    }
}

fn enter_student(students: &mut HashMap<String, f32>, num: u8) {
    let mut input = String::new();
    println!("Please enter student {}'s name: ", num);
    stdin().read_line(&mut input).unwrap();
    let name = input.trim();
    println!("Please enter student {}'s grade: ", num);
    let mut input_two = String::new();
    stdin().read_line(&mut input_two).unwrap();
    let grade: f32 = input_two.trim().parse().unwrap();
    students.insert(name.to_owned(), grade);
}
