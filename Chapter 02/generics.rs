use std::fmt::Display;
use std::cmp::{Ord, Ordering};

fn greater<T: Ord>(one: T, two: T) -> T{
   return match one.cmp(&two){
	Ordering::Less => two, 
	Ordering::Greater => one, 
	Ordering::Equal => one, 
   }
}

fn greater_alt<T>(one: T, two: T)
where T: Ord + Display{
    match one.cmp(&two){
        Ordering::Less => println!("{} is greater", two), 
        Ordering::Greater => println!("{} is greater", one), 
        Ordering::Equal => println!("They are equal"), 
       }
}




fn main(){
    let g = greater(1, 4);
    println!("{}", g);
    greater_alt(7, 8);
}
