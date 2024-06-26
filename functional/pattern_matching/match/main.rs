fn main(){
    let x = Some(1);
    let y = match x {
        None => 0,
        Some(i) => i + 1,
    };
    println!("{y}");

    let dice_roll = 9;
    let result = match dice_roll {
        3 => 1,
        7 => 2,
        _ => 3, // has to be exhaustive
    };
    println!("{result}");

    let z = 1;

    match z {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let w = 5;

    match w {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let v = 'c';
    match v {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }

    // match guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"), // doesn't check for exhaustiveness
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // bindings
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7, // @ creates a variable id_variable and tests range
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => { // without @ cannot save id_variable for print statement
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}