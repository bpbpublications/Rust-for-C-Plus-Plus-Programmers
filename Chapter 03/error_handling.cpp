// In this program we will attempt to login 
#include<iostream>
using namespace std;

void login(int password){
    try
    {
        // our actual password 
        int actual = 97979;
        // does our guess equal the actual password 
        if(password == actual){
            // if so we are logged in 
            cout << "Logged in successfully!" << endl;
        } else {
            // if not, throw that password away 
            throw(password);
        }
    }
    // catch the password 
    catch(int password)
    {
        // you are denied logging in 
        cout << "Login denied: " << password << " is wrong!" << endl;
    }
}

int main(){
    login(8888);
    return 0;
}

// Output: 
// Login denied: 8888 is wrong!