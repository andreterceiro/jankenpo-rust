fn read_user_input() -> str {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Please enter some text: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}

fn main() {
    println!("Jankenpo!!!\n");
    println!("Please type 'p' for 'paper', 'r' for 'rock' or 's' for 'scissors':");
    let mut userChoice = "";
    
    while userChoice != "p" && userChoice != "r" && userChoice != "s" {
        userChoice = read_user_input();
        //println!("{}", userChoice);
    }
}