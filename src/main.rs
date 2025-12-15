use std::io;

fn main() {

    println!("Welcome to the game!\nWhat is the adventurer's name? ");

    let mut name = String::new();

    io::stdin()
        .read_line( &mut name)
        .expect("Failed to read input");

    // main gameplay loop
    loop {
        println!("Greetings, {name}What is your next task?:");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

        let input = input.trim();

        if input == "quit" {
            println!("Thank you for playing!");
            break;
        }
        else {
            println!("Not a valid command")
        }

    }
}