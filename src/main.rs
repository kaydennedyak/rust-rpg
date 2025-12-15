use std::io;
use rand::Rng;

fn main() {

    struct Player {
        name: String,
        gold: u16,
    }

    println!("<> | Welcome to the game!\nWhat is the adventurer's name? ");

    let mut name = String::new();

    io::stdin()
        .read_line( &mut name)
        .expect("Failed to read input");

    let mut player = Player {
    name: name.trim().to_string(),
    gold: 0,
    };

    // main gameplay loop
    loop {
        println!("<> | Greetings, {}! What is your next task?\n('dig', 'quit') : ", player.name);

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

        let input = input.trim();

        if input == "quit" {
            println!("<> | Thank you for playing!");
            break;
        }
        else if input == "dig" {
            let mut rng = rand::rng();
            let chance: u8 = rng.random_range(1..=100);

            if chance <= 40 {
                let reward = rng.random_range(1..=10);
                player.gold += reward;
                println!("You dug and found {} gold! You now have {} total gold.", reward, player.gold);
               
            } else {
                println!("You dug and found nothing!");
            }
        }
        else {
            println!("<> | Not a valid command")
        }

    }
}