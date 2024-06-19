// g++ class.cpp -o class
// .\class.exe

#include <iostream>
#include <vector>
#include <array>
#include <string>

class Employee {
public:
    // Constructor
    Employee(int id, const std::string& name, const std::vector<int>& stats, const std::array<int, 1>& jersey)
        : Id(id), Name(name), Stats(stats), Jersey(jersey) {}

    // Print method
    void print() const {
        std::cout << "Employee { Id: " << Id << ", Name: " << Name << ", Stats: [";
        for (const auto& s : Stats) {
            std::cout << s << " ";
        }
        std::cout << "], Jersey: [" << Jersey[0] << "] }\n";
    }

    // Getters and setters
    int getId() const { return Id; }
    void setId(int id) { Id = id; }

    std::string getName() const { return Name; }
    void setName(const std::string& name) { Name = name; }

    std::vector<int> getStats() const { return Stats; }
    void setStats(const std::vector<int>& stats) { Stats = stats; }

    std::array<int, 1> getJersey() const { return Jersey; }
    void setJersey(const std::array<int, 1>& jersey) { Jersey = jersey; }

private:
    int Id;
    std::string Name;
    std::vector<int> Stats;
    std::array<int, 1> Jersey;
};

int main() {
    // init class
    Employee e1(3, "freeman", {3, 4, 5}, {5});

    // update
    e1.setId(1);
    e1.setName("ohtani");
    std::vector<int> stats = e1.getStats();
    stats[0] = 1;
    stats[1] = 2;
    stats[2] = 3;
    stats.push_back(4);
    e1.setStats(stats);
    e1.setJersey({17});

    // print
    e1.print(); // Employee { Id: 1, Name: ohtani, Stats: [1 2 3 4], Jersey: [17] }

    return 0;
}