// rustc struct.rs 
// .\struct.exe

#[derive(Clone, Debug)]
struct Employee {
    id: i32,
    name: String, // immutable utf-8 encoded
    stats: Vec<i32>,
    jersey: [i32; 1],
}

// Function to perform a deep copy of the Employee struct
fn deep_copy(e: &Employee) -> Employee {
    Employee {
        id: e.id,
        name: e.name.clone(), // string as immutable, shallow copy is fine
        stats: e.stats.clone(), // deep copy of the stats vector
        jersey: e.jersey, // array is copied by value
    }
}

fn main() {
    // Initialize struct
    let mut e1 = Employee {
        id: 3,
        name: "freeman".to_string(), // just a pointer to string
        stats: vec![3, 4, 5], // deep copied with clone
        jersey: [5],
    };

    // deep copy
    let e2 = deep_copy(&e1);

    // shallow copy - is deep for strings and vectors
    let e3 = e1.clone();

    // Update
    e1.id = 1;
    e1.name = "ohtani".to_string();
    e1.stats[0] = 1;
    e1.stats[1] = 2;
    e1.stats[2] = 3;
    e1.stats.push(4);
    e1.jersey[0] = 17;

    // Print
    println!("{:?}", e1); // Employee { id: 1, name: ohtani, stats: [1, 2, 3, 4], jersey: [17] }
    println!("{:?}", e2); // Employee { id: 3, name: freeman, stats: [3, 4, 5], jersey: [5] }
    println!("{:?}", e3); // Employee { id: 3, name: freeman, stats: [3, 4, 5], jersey: [5] }
}