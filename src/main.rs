use std::io;
use colored::*;
use rand::RngExt;

const EASY_LUCKY_CHANCE: f64 = 0.75;
const EASY_START_MONEY: u64 = 2500;
const NORMAL_LUCKY_CHANCE: f64 = 0.5;
const NORMAL_START_MONEY: u64 = 750;
const HARD_LUCKY_CHANCE: f64 = 0.25;
const HARD_START_MONEY: u64 = 250;

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
            "1"|"play" => {println!(); modes(); break;}
            "2"|"rules" => description(),
             _  => {println!("{}", "Try to write [1] or [2]".red()); println!(); continue;}
        }  
    }
}

fn modes() {
    let mut user_input = String::new();
    
    loop {
        println!();
        println!("1)Easy, start money: {EASY_START_MONEY}, risk of bad luck: 25%, mod: If you're unlucky, you won't lose everything, but your money will be split into two");
        println!("2)Normal, start money: {NORMAL_START_MONEY}, risk of bad luck: 50%");
        println!("3)Hard, start money: {HARD_START_MONEY}, risk of bad luck: 75%");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => print!(""),
            Err(er) => println!("Program crash, error: {er}")
        }

        match &*user_input.trim().to_lowercase() {
            "1"|"easy" => {let mode="easy"; game(mode); break},
            "2"|"normal" => {let mode="normal"; game(mode); break},
            "3"|"hard" => {let mode="hard"; game(mode); break},
            _ => {println!("{}", "Try to write [1] or [2] or [3]".red()); continue}
        }
    }
}

fn game(mode: &str) {
    let mut rng = rand::rng();
    let mut user_input = String::new();
    
    println!();
    let mut money = match mode {
        "easy" => EASY_START_MONEY,
        "normal" => NORMAL_START_MONEY,
        "hard" => HARD_START_MONEY,
        _ => todo!()
    };

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
                
                let lucky = match mode {
                    "easy" => rng.random_bool(EASY_LUCKY_CHANCE),
                    "normal" => rng.random_bool(NORMAL_LUCKY_CHANCE),
                    "hard" => rng.random_bool(HARD_LUCKY_CHANCE),
                    _ => todo!()
                };
                
                match lucky {
                    true => {
                        money *= 2;

                        println!("{}", "Good news! I double your money".green().bold());
                        println!("Now, your money: {money}$");
                    },
                    false => {
                        match mode {
                            "easy" => {
                                money /= 2;
                                println!("I theft part of your money!");
                            }
                            _=> {
                                money = 0;                                
                                println!("{}", "Bad news! I theft you money".red().bold());                         
                            }
                        }
                       
                        println!("Now, your money: {money}$");                           
                        
                        let user_continue = game_continue();
                        
                        money = match mode {
                            "easy" => EASY_START_MONEY,
                            "normal" => NORMAL_START_MONEY,
                            "hard" => HARD_START_MONEY,
                            _ => todo!()
                        };            
                        
                        match user_continue {
                            true => continue,
                            false => break
                        }                    
                    }
                }   
            },
            "2"|"safe" => {
                
                match mode {
                    "easy" => {
                        if money == EASY_START_MONEY {
                            println!("{}", "At least you didn't lose anything".purple().bold())
                        } else if money > EASY_START_MONEY {
                            let congratulation = format!("Smart choise, you win {money}$");
                            println!("{}", congratulation.green().bold())
                        } else if money < EASY_START_MONEY {
                            let congratulation = format!("It's sad, but you're in the minus, your money: {money}");
                            println!("{}", congratulation.red().bold())
                        } else {
                            let congratulation = format!("There was a glitch in the program, but at least your money: {money}");
                            println!("{}", congratulation.purple().bold())
                        }
                    },
                    "normal" => {
                        if money == NORMAL_START_MONEY {
                            println!("{}", "At least you didn't lose anything".purple().bold())
                        } else if money > NORMAL_START_MONEY {
                            let congratulation = format!("Smart choise, you win {money}$");
                            println!("{}", congratulation.green().bold())
                        } else if money < NORMAL_START_MONEY {
                            let congratulation = format!("It's sad, but you're in the minus, your money: {money}");
                            println!("{}", congratulation.red().bold())
                        } else {
                            let congratulation = format!("There was a glitch in the program, but at least your money: {money}");
                            println!("{}", congratulation.purple().bold())
                        }
                    },
                    "hard" => {
                        if money == HARD_START_MONEY {
                            println!("{}", "At least you didn't lose anything".purple().bold())
                        } else if money > HARD_START_MONEY {
                            let congratulation = format!("Smart choise, you win {money}$");
                            println!("{}", congratulation.green().bold())
                        } else if money < HARD_START_MONEY {
                            let congratulation = format!("It's sad, but you're in the minus, your money: {money}");
                            println!("{}", congratulation.red().bold())
                        } else {
                            let congratulation = format!("There was a glitch in the program, but at least your money: {money}");
                            println!("{}", congratulation.purple().bold())
                        }                        
                    }
                    _ => println!("Error")
                }

                let user_continue = game_continue();
                
                money = match mode {
                    "easy" => EASY_START_MONEY,
                    "normal" => NORMAL_START_MONEY,
                    "hard" => HARD_START_MONEY,
                    _ => todo!()
                };
                
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
