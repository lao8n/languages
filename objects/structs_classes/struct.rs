// rustc struct.rs
// .\struct.exe

#[derive(Debug)]
struct Employee {
    id: i32,
    name: String,
    stats: Vec<i32>,
    jersey: [i32; 1],
}

fn main() {
    // Initialize struct
    let mut e1 = Employee { // needs to be mutable
        id: 3,
        name: "freeman".to_string(),
        stats: vec![3, 4, 5],
        jersey: [5],
    };

    // Update fields
    e1.id = 1;
    e1.name = "ohtani".to_string();
    e1.stats[0] = 1;
    e1.stats[1] = 2;
    e1.stats[2] = 3;
    e1.stats.push(4);
    e1.jersey[0] = 17;

    // Print
    println!("{:?}", e1); // Employee { id: 1, name: "ohtani", stats: [1, 2, 3, 4], jersey: [17] }
}