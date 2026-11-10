#include <cstdlib>
#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <filesystem>
#include <unistd.h>
#include <sys/wait.h>
#include "utils.hpp"

using namespace std;
namespace fs = std::filesystem;

const string INPUT_PREFIX = "\\input{";

string parseInput(const string& line) {
    if (!line.starts_with(INPUT_PREFIX)) return "";

    size_t end = line.find('}', INPUT_PREFIX.length());
    if (end == string::npos) return "";

    return line.substr(INPUT_PREFIX.length(), end - INPUT_PREFIX.length());
}

int countInFile(const fs::path& path, const string& word) {
    ifstream file(path);
    
    if (!file.is_open()) {
        cerr << "Cannot open file " << path << '\n';
        return 0;
    }

    int count = 0;
    vector<pid_t> children;
    string line;

    while (getline(file, line)) {
        string included = parseInput(line);

        if (included.empty()) {
            count += countWord(line, word);
            continue;
        }

        cout.flush();
        cerr.flush();

        pid_t pid = fork();

        if (pid == -1) {
            perror("fork");
            exit(1);
        }

        if (pid == 0) {
            int res = countInFile(path.parent_path() / included, word);
            _exit(res);
        }

        children.push_back(pid);
    }

    file.close();

    for (pid_t pid : children) {
        int status;
        if (waitpid(pid, &status, 0) == -1) {
            perror("waitpid");
            continue;
        }

        if (WIFEXITED(status)) {
            count += WEXITSTATUS(status);
        } else {
            cerr << "Child " << pid << " terminated abnormally\n";
        }
    }

    return count;
}

int main(int argc, char** argv) {
    if (argc != 3) {
        cerr << "Incorrect args. Use " << argv[0] << " <startFile> <word>\n";
        return 2;
    }

    cout << countInFile(argv[1], argv[2]) << '\n';
    return 0;
}
