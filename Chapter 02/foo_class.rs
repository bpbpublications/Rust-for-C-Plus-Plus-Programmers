struct Foo{
    // members => fields
    bar: i32, 
    baz: i32, 
}

// bounded methods => implementations 
// to declare use keyword impl
impl Foo{
    // to create instance return Self 
    // to use instance use self, &self or &mut self in the parameters 

    // constructor 
    pub fn new()-> Self{
        Foo{
            bar: 45, 
            baz: 32
        }
    }
    // bounded method
    pub fn do_something(&mut self){
        self.bar = self.bar + self.baz;
        self.baz -= 1;
    }
}

// structs will automatically be deallocated 
// but we can still create a destructor by
// implementing Drop for Foo
impl Drop for Foo{
    fn drop(&mut self){
        println!("Bye {} and {}", self.bar, self.baz)
    }
}

fn main(){
    let mut foo = Foo::new();
    foo.do_something();
}