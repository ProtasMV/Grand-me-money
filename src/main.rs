use std::io;
use colored::*;
use rand::RngExt;

fn main() {
    let mut user_input = String::new();

    println!("{}", "Welcome to Duble my money game!".green().bold());
    println!("1)Play, 2)Rules");

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => print!(""),
        Err(er) => println!("Program crash, error: {er}")
    }

    if user_input.trim() == "1" {
        game();
    } else if user_input.trim() == "2" {
        description();
    }
}

fn game() {
    let mut rng = rand::rng();
    let mut user_input = String::new();
    let mut money = 100;

    println!("Your money: {money}");

    loop {

        if money <= 0 {
            println!("{}", "I am sorry guy, but you didn't win anything".red());
            println!(" "); 
            
            let user_continue = game_continue();
            if user_continue == true {
                money = 100;
                continue;
            } else if user_continue == false {
                break;
            }        
        }

        println!(" ");        
        println!("Risk or safe?");
        println!("tip: 1)Risk, 2)Safe");
            
        user_input.clear();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => print!(""),
            Err(er) => println!("Program crash, error: {er}")
        }

        if user_input.trim() == "1" || user_input.trim().to_lowercase() == "risk" {
            let lucky = rng.random_range(1..=2);
            user_input.clear();
            
            if lucky == 1 {
                user_input.clear();
                money *=2;

                println!("Congratulations! I double your money");
                print!("{}", "Now, your money: ".green());
                println!("{money}");
                } else if lucky == 2 {
                money = 0;
                    
                println!(" ");
                println!("{}", "Bad News, I theft your money");
                print!("{}", "Now, your money: ".red());
                println!("{money}");
                } else {
                println!("Random error, random number: {lucky}");
            }
        } else if user_input.trim() == "2" || user_input.trim().to_lowercase() == "safe" {
            user_input.clear();
            match money {
                0 => println!("{}", "I am sorry guy, but you didn't win anything".red().bold()),
                100 => println!("{}", "Apparently, you've decided to give up the game.".blue().bold()),
                _ => {print!("{}", "You win: ".green().bold()); print!("{money}"); println!("{}", "$".green().bold());}
            }

            let user_continue = game_continue();
            if user_continue == true {
                money = 100;
                continue;
            } else if user_continue == false {
                break;
            }

        } else {
            println!("Input error, please try [1] or [2]"); user_input.clear(); continue;
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
    println!("{}", "Press anything key to continue".blue());

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => print!(" "),
        Err(er) => println!("Program crash, error: {er}")
    }

    main();
}

fn game_continue() -> bool{
    let mut user_input = String::new();

    loop {
        println!("You want play again? 1)Yes, 2)No");
                
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => print!(""),
            Err(er) => println!("Program crash, error: {er}")
        }
        
        match user_input.trim() {
            "1" => {user_input.clear(); break true},
            "2" => {user_input.clear(); break false},
            _  => {user_input.clear(); println!("Please write [1] or [2]"); println!(" "); continue}
        };
    }
}