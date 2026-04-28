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
let mut cheats= false;
    
    loop {
        let mut user_input = String::new();

        println!("{}", "Welcome to Duble my money game!".green().bold());
        println!("1)Play, 2)Rules, 3)Settings");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {},
            Err(er) => println!("Program crash, error: {er}")
        }

        match &*user_input.trim().to_lowercase() {
            "1"|"play" => {modes(cheats); break;}
            "2"|"rules" => description(),
            "3"|"settings" => cheats = settings(cheats),
             _  => {println!("{}", "Try to write [1] or [2]".red()); println!(); continue;}
        }  
    }
}

fn modes(cheats: bool) {
    let mut user_input = String::new();
    
    loop {
        println!();
        println!("1)Easy, start money: {EASY_START_MONEY}, risk of bad luck: 25%, mod: If you're unlucky, you won't lose everything, but your money will be split into two");
        println!("2)Normal, start money: {NORMAL_START_MONEY}, risk of bad luck: 50%");
        println!("3)Hard, start money: {HARD_START_MONEY}, risk of bad luck: 75%");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {},
            Err(er) => println!("Program crash, error: {er}")
        }

        match &*user_input.trim().to_lowercase() {
            "1"|"easy" => {let mode="easy"; game(mode, cheats); break},
            "2"|"normal" => {let mode="normal"; game(mode, cheats); break},
            "3"|"hard" => {let mode="hard"; game(mode, cheats); break},
            _ => {println!("{}", "Try to write [1] or [2] or [3]".red()); continue}
        }
    }
}

fn game(mode: &str, cheats: bool) {
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

        let lucky = match mode {
            "easy" => rng.random_bool(EASY_LUCKY_CHANCE),
            "normal" => rng.random_bool(NORMAL_LUCKY_CHANCE),
            "hard" => rng.random_bool(HARD_LUCKY_CHANCE),
            _ => todo!()
        };

        match cheats {
            true => {print!("[Cheats]: "); if lucky == true {println!("{}", "luck".green())} else if lucky == false {println!("{}", "unluck".red())}}
            false => {}
        }

        user_input.clear();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {},
            Err(er) => println!("Something went wrong, error: {er}")
        }
        println!();

        match &*user_input.trim().to_lowercase() {
            "1"|"risk" => {
                
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
                                println!("{}", "I theft part of your money!".red().bold());
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
The rules of the game are simple: you deposit an initial amount,
you can take a risk, and you have a 50% chance of doubling your winnings or losing everything.
Trust your intuition, good luck!");

    println!(" ");
    println!("{}", "Press anything key to continue".blue().bold());

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {},
        Err(er) => println!("Program crash, error: {er}")
    }

}

fn settings(mut cheats: bool) -> bool {
    let mut user_input = String::new();
    loop {
        println!(" ");
        print!("{}", "1)Cheats: ".bold());
        
        match cheats {
            true => println!("{}", "True".green()),
            false => println!("{}", "False".red())
        }

        println!(" ");
        println!("What setting would you like to change? [print [No] for exit]");
        
        user_input.clear();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {},
            Err(er) => {println!("Program crash, error: {er}")}
        }

        match &*user_input.trim().to_lowercase() {
            "1"|"cheats" => {
                user_input.clear();
                println!("[Cheats]: 1)True, 2)False");
                
                match io::stdin().read_line(&mut user_input) {
                    Ok(_) => {},
                    Err(er) => {println!("Program crash, error: {er}")}
                }

                match &*user_input.trim().to_lowercase() {
                    "1"|"true" => {user_input.clear(); cheats = true},
                    "2"|"false" => {user_input.clear(); cheats = false},
                    _ => {user_input.clear(); println!("Try to write [1] or [2]");}
                }
            },
            "no"|"n" => {
                break cheats;
            },
            _ => {
                user_input.clear();

                println!("{}", "Try to write [1] or [no]".red());
                continue;
            }
        }
        println!("where are we heading next? 1)Settings menu, 2)Main menu");  
        
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {},
            Err(er) => {println!("Program crash, error: {er}")}
        }    
        
        match &*user_input.trim().to_lowercase() {
            "1"|"settings menu"|"settings" => {user_input.clear(); continue},
            "2"|"main menu"|"main" => {user_input.clear(); if cheats == true {break true} else if cheats == false {break false}},
            _=> todo!()
        }
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
            "1"|"yes" => {user_input.clear(); println!(); break true},
            "2"|"no" => {user_input.clear(); println!(); break false},
            _  => {user_input.clear(); println!("{}", "Please write [1] or [2]".red()); println!(" "); continue}
        };
    }
}
