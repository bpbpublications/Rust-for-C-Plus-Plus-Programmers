pub struct Foo{
    x: i32 
}

impl Foo{
    // Constructor 
    pub fn new(x: i32) -> Self{
        // if param matches field name, 
        // we do not need to do Foo{x: x}
        Self{x}
    }
    // public function 
    // can be accessed as Foo.update()
    pub fn update(&mut self){
        self.update_self()
    }
    // private function 
    // cannot be accessed outside of impl block
    fn update_self(&mut self){
        self.x = (self.x * 2) + 5;
    }
}

fn main(){
    let mut foo = Foo::new(11);
    println!("Foo is {}", &foo.x);
    foo.update();
    println!("Updated foo to: {}", &foo.x)
}