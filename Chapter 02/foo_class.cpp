#include<iostream>
class Foo{
    private: 
        // members
        int bar;
        int baz;
    // constructor
    public:
    Foo(){
        bar = 45;
        baz = 32;
    }
    // bounded method
    void do_something(){
        bar = baz + bar;
        baz--;
    }
    // destructor
    ~Foo(){
        // integers don't need to be freed 
        std::cout << "Bye " << bar << " and " << baz << std::endl;
    }

};

int main(){
    Foo f = Foo();
    f.do_something();
}