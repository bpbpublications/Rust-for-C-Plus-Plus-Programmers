macro_rules! vector{
    ($($element: expr), *) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($element);
            )*
            vec
        }
    };
}


fn main(){
    let vec = vector![98, 87, 30, 60];
    /* Expands to 
    let vec = {
        let mut vec = Vec::new();
        vec.push(98);
        vec.push(87);
        vec.push(30);
        vec.push(60);
        vec
    }*/
    println!("{:?}", vec)
}