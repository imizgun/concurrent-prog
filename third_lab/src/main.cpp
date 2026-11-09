#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <fstream>
#include <string>

using namespace std;


int main(int argc, char** argv) {
    if (argc != 3) {
        cout << "Incorrect args. Use <program> <startFile> <word>";
        exit(2);
    }

    ifstream startFile(argv[1]);
    string line, content;
    
    while (getline(startFile, line)) {
        content += line + '\n';
    }

    startFile.close();

    cout << content;
}
