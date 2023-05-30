use colored::*;
use std::io::{stdin, stdout, Write};

// сновная функция
fn main() {
    title();

    let private: &str = ask("input".bright_blue(), "Please enter your wallet's private key : ");



    println!("Your private key is {}", private);

    // Example wallet :
    // priv : L5Mhy9BaJrs4SosqiHsL8QECb4Ft3Ct8Gqiw2sMiF8QzgnVKrvkA
    // pub : 03ae70e44e86aacf01fe41778c0ba286ad29ac239a335e3ed3ed731311a80acdc2
    // addrs : 1P1WCK6rvpfmxYySHAtuhM6UbSL5PuXK74
    let _descriptor: &str = "pkh(03ae70e44e86aacf01fe41778c0ba286ad29ac239a335e3ed3ed731311a80acdc2)";

    println!("")
}

// Функция отображения заголовка
fn title() {
    let title: &str = "
┌──────────────────────────────────┐
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
            print!("{}", c.to_string().bright_blue())
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
