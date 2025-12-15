use std::{io, time::Duration};
use rand::Rng;

fn main() {
    struct Player {
        name: String,
        gold: u16,
        activity_timeout: u8,
    }

    println!("<> | Welcome to the game!\nWhat is the adventurer's name? ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let mut player = Player {
        name: name.trim().to_string(),
        gold: 0,
        activity_timeout: 0,
    };

    // Main gameplay loop
    loop {
        if player.activity_timeout > 0 {
            println!("You must wait {} seconds... returning home!", player.activity_timeout);
            std::thread::sleep(Duration::from_secs(player.activity_timeout.into()));
            player.activity_timeout = 0;
        }

        println!("<> | Greetings, {}! What is your next task?\n('dig', 'quit') : ", player.name);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if input == "quit" {
            println!("<> | Thank you for playing!");
            break;
        } else if input == "dig" {
            let mut rng = rand::rng();
            let chance: u8 = rng.random_range(1..=100);

            if chance <= 60 {
                let reward = rng.random_range(1..=20);
                player.gold += reward;
                println!("You dug and found {} gold! You now have {} total gold.", reward, player.gold);
                player.activity_timeout = 2;
            } else {
                println!("You dug and found nothing!");
                player.activity_timeout = 1;
            }
        } else {
            println!("<> | Not a valid command");
        }
    }
}
