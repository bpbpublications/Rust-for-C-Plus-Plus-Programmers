macro_rules! multi{
    ($($f: ident), +) => ({
        $(
            $f();
        )+
    });
}

fn hello(){
    println!("Hello")
}

fn bye(){
    println!("Bye")
}

fn hello_bye(){
    hello();
    bye();
}

fn main(){
    multi![hello, bye, hello_bye]
}

