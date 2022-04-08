use std::process;
use std::io::{self, BufRead};

pub fn ui() {
  welcome();
  loop {
    question();
  }
}

fn again() { 
    println!("Press \'Y\' to Yes");
    println!("Press \'N\' to No");
    println!("Your Choice: ");
    let choice = io::stdin().lock().lines().next().unwrap().unwrap();
    println!("---------------------------------");
    if choice == "Y" || choice == "y"{
        question();
    } else if choice == "N" || choice == "n" {
        quit();  
    } else {
        println!("Sorry, I don't get it.");
        again();
    }
}

fn question() {
    println!("Press \'E\' to encode");
    println!("Press \'D\' to decode");
    println!("Press \'Q\' to quit");
    println!("Your Choice: ");
    let choice = io::stdin().lock().lines().next().unwrap().unwrap();
    println!("---------------------------------");
    if choice == "E" || choice == "e" {
        encode();
        println!("Yay! That was awesome ... let's do it one more time?");
        again();
    } else if choice == "D" || choice == "d" {
        decode();
        println!("Yay! That was awesome ... let's do it one more time?");
        again();
    } else if choice == "Q" || choice == "q" {
        quit();
    } else {
        println!("Uh, Oh .. You mean what?");
        question();
    }
}

fn quit() {
    println!("See you soon!");
    process::exit(0);
}

fn decode() {
    println!("You chode decode");
}

fn encode() {
    println!("You chode encode");
}

fn welcome() {
    println!("Hello and Welcome to the very best Bech32m decoding tool out there!");
    println!("Tell me, what is your wish ...");
}