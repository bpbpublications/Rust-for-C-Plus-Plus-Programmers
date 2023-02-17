#[deny(unused_variables)]
fn two(a: u32) -> u32 {
    2
}

// by default non-snake-case is warn 
// functions should be snaked case 
#[deny(non_snake_case)]
pub mod foo{
    // allows non snake case 
    #[allow(non_snake_case)]
    fn functionOne() {}
    // wanrs of non snake case 
    #[warn(non_snake_case)]
    fn functionTwo(){}
    // must be snaked cased 
    fn function_three(){}
}

fn main(){
    let two = two(3);
    println!("{two}")
}





