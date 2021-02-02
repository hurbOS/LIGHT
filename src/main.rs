extern crate colorful;
use std::env;
use colorful::Color;
use colorful::Colorful;
mod commands;

fn help() {
    let s = " 
     _      _____ _____ _    _ _______ 
    | |    |_   _/ ____| |  | |__   __|
    | |      | || |  __| |__| |  | |   
    | |      | || | |_ |  __  |  | |   
    | |____ _| || |__| | |  | |  | |   
    |______|_____\\_____|_|  |_|  |_|   


           ";
    println!("{}", s.color(Color::Yellow)); 
    print!("Commands:

           Help: --help
           Shows this menu.

           Update: --update/-u <optional>
           Update repositories, search for package updates, optionally specify packages.

           Install: --install/-i <package>
           Search, download, and install a package from enabled repositories.");
}

fn command_parse() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    match &command[..] {
        "--help" => {
            help();
        },
        "-i" | "--install" => {
            commands::install();
        }
        "-u" | "--update" => {
            commands::update();
        }
        _ => {
            print!("Invalid command, use --help\n");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            command_parse();
        },
        _ => {
            help();
        }
    }
}
