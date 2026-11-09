#include <cstdlib>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

using namespace std;


int main(int argc, char** argv) {
    if (argc != 3) {
        cout << "Incorrect args. Use <program> <startFile> <word>";
        exit(2);
    }

    ifstream startFile(argv[1]);

    if (!startFile.is_open()) {
        cout << "File wasn't open";
        exit(1);
    }
    
    stringstream buffer;
    buffer << startFile.rdbuf();
    string content = buffer.str();

    startFile.close();

    cout << content;
}
