use std::fs;
use curl::easy::Easy;
use colorful::Color;
use colorful::Colorful;

pub fn update() {
    print!("Updating Repositories");
    let mut easy = Easy::new();
    easy.url("http://127.0.0.1/repoheader").unwrap();
    easy.write_function(|data| {
        fs::write("repoheader", data).expect("Unable to read file");
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap()
}

pub fn install() {
    update();
    let mut easy = Easy::new();
    easy.url("http://127.0.0.1/test.repo").unwrap();
    easy.write_function(|data| {
        fs::write("test.repo", data).expect("Unable to read file");
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap()
}

pub fn help() {
   let s = " 
     _      _____ _____ _    _ _______ 
    | |    |_   _/ ____| |  | |__   __|
    | |      | || |  __| |__| |  | |   
    | |      | || | |_ |  __  |  | |   
    | |____ _| || |__| | |  | |  | |   
    |______|_____\\_____|_|  |_|  |_|   


           ";
    println!("{}", s.color(Color::Yellow)); 
    print!("    Commands:

           Help: -h/--help
           Shows this menu.

           Update: --update/-u <optional>
           Update repositories, search for package updates, optionally specify packages.

           Install: --install/-i <package>
           Search, download, and install a package from enabled repositories.
           ");
}