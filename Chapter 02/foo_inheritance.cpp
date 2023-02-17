#include<iostream>
using namespace std;
class Instructions{
    public: 
    virtual void d_up(){cout<<"Moving up!" << endl;}
    virtual void d_down(){cout<<"Moving down!" << endl;}
    virtual void d_left(){cout<<"Moving left!" << endl;}
    virtual void d_right(){cout<<"Moving up!" << endl;}
};
// Robot inherits Instructions 
class Robot: public Instructions{
    // let's have our robot have inversed 
    // left and right instructions 
    public: 
     void d_left() override{cout << "Moving right!" << endl;}
     void d_right() override{cout << "Moving left!" << endl;}
};
int main(){
    Instructions ins;
    Robot robot;
    Instructions* i = &ins;
    cout << "Instructions: " << endl;
    i->d_left();
    i->d_right();
    i = &robot;
    cout << "Robot: " << endl;
    i->d_left();
    i->d_right();
    return 0;
}