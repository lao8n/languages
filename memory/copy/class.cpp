// g++ class.cpp - o class
// .\class.exe
#include <iostream>
#include <string>
#include <vector>
#include <array>

class Employee {
public:
    // constructor
    Employee(int id, const std::string& name, const std::vector<int>& stats, const std::array<int, 1>& jersey)
        : Id(id), Name(name), Stats(stats), Jersey(jersey) {}

    // rule of 3
    // 1. destructor 
    ~Employee(){
        // nothing to clean up as std::string and std::vector handle this themselves
        // if we had dynamically allocated memory we might have had to 
        // delete[] _data;
    }

    // 2. copy constructor
    Employee(const Employee& other)
        : Id(other.Id), Name(other.Name), Stats(other.Stats),  Jersey(other.Jersey) {
            // if we had to do a deep copy of some dynamically allocated memory
            // _size = source._size;
            // _data = new int[_size];
            // *_data = *source._data; // deep copy
        }

    // 3. assignment operator
    Employee& operator=(const Employee& other) {
        if (this != &other) { // protection against self-assignment
            Id = other.Id;
            Name = other.Name;
            Stats = other.Stats; // to achieve shallow copy of vector would need to use shared pointer
            Jersey = other.Jersey;
        }
        return *this;
    }

    void print() const {
        std::cout << "Id: " << Id << ", Name: " << Name << ", Stats: [";
        for (const auto& s : Stats) {
            std::cout << s << " ";
        }
        std::cout << "], Jersey: [" << Jersey[0] << "]\n";
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
    // init object
    Employee e1(3, "freeman", {3, 4, 5}, {5});

    // deep copy
    Employee e2 = e1; // not possible to do deep copy

    // shallow copy - but same as deep copy as everything deep copied
    Employee e3 = e1;

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
    std::cout << "e1: ";
    e1.print(); //  Id: 1, Name: ohtani, Stats: [1 2 3 4 ], Jersey: [17]
    std::cout << "e2: ";
    e2.print(); // Id: 3, Name: freeman, Stats: [3 4 5 ], Jersey: [5]
    std::cout << "e3: ";
    e3.print(); // Id: 3, Name: freeman, Stats: [3 4 5 ], Jersey: [5]

    return 0;
}