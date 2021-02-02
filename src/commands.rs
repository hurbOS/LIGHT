use std::fs;
use curl::easy::Easy;

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
