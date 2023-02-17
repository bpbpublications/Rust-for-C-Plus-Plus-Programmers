// Our program will attempt to open a file 
#include<stdio.h>
#include<errno.h>

void attemptFile(FILE* file, char* path){
    int errnum;
    file = fopen(path, "rb");
    // check if the file is NULL
    if(file == NULL){
        // we have an error 
        // Use perror()
        perror("Error opening file");
    } else{
        // no error so we can safely close the file
        printf("File exists, closing...");
        fclose(file);
    }
}

int main(){
    FILE* file;
    attemptFile(file, "foo.txt");
    //return 0;
}

// Output: 
// Error opening file: No such file or directory