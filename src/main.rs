use std::io;
use colored::*;
use rand::RngExt;

fn main() {
    loop {
        let mut user_input = String::new();

        println!("{}", "Welcome to Duble my money game!".green().bold());
        println!("1)Play, 2)Rules");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => print!(""),
            Err(er) => println!("Program crash, error: {er}")
        }

        match &*user_input.trim().to_lowercase() {
            "1"|"play" => {println!(); game(); break;}
            "2"|"rules" => {println!(); description();}
             _  => {println!("{}", "Try to write [1] or [2]".red()); println!(); continue;}
        }
    }
}

fn game() {
    let mut rng = rand::rng();
    let mut user_input = String::new();
    let mut money = 100;

    println!("Money: {money}");
    loop {
        
        println!("{}", "Risk or safe?".blue().bold());
        println!("Tip: 1)Risk, 2)Safe");
        
        user_input.clear();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => print!(" "),
            Err(er) => println!("Something went wrong, error: {er}")
        }
        println!();

        match &*user_input.trim().to_lowercase() {
            "1"|"risk" => {
                let lucky = rng.random_bool(0.5);
                
                match lucky {
                    true => {
                        money *= 2;

                        println!("{}", "Good news! I double your money".green().bold());
                        println!("Now, your money: {money}$");
                    },
                    false => {
                        money = 0;

                        println!("{}", "Bad news! I theft you money".red().bold());
                        println!("Now, your money: {money}$");
                        
                        let user_continue = game_continue();
                        match user_continue {
                            true => {money = 100; continue},
                            false => break
                        }                    
                    }
                }   
            },
            "2"|"safe" => {
                match money {
                    100 => println!("{}", "Apparently you decided not to try your luck".purple().bold()),
                     _  => {print!("{}", "Smart choise! You win: ".purple().bold()); print!("{money}"); println!("{}", "$".purple().bold())}
                }

                let user_continue = game_continue();
                money = 100;
                match user_continue {
                    true => continue,
                    false => break
                }

            },
            _ => {
                println!("{}", "Try to write [1] or [2]".red());
                println!();
                continue;
            }
        }
    }
}

fn description() {
    let mut user_input = String::new();
    
    println!(" ");
    println!("
The rules of the game are simple: you deposit a starting amount of $100,
you can take a risk and have a 50% chance of doubling the amount or losing everything.
Focus on your own intuition, good luck!");

    println!(" ");
    println!("{}", "Press anything key to continue".blue().bold());

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => print!(" "),
        Err(er) => println!("Program crash, error: {er}")
    }

}

fn game_continue() -> bool{
    let mut user_input = String::new();

    loop {
        println!("{}", "You want play again? 1)Yes, 2)No".blue());
                
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => print!(""),
            Err(er) => println!("Program crash, error: {er}")
        }
        
        match &*user_input.trim().to_lowercase() {
            "1"|"yes" => {user_input.clear(); println!();break true},
            "2"|"no" => {user_input.clear(); println!();break false},
            _  => {user_input.clear(); println!("{}", "Please write [1] or [2]".red()); println!(" "); continue}
        };
    }
}
