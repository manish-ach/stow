use std::process::Command;
use std::io::{self,Write};
fn main() {
    println!("hi");
    let output = {
        Command::new("ls")
        .arg("-a")
        .output()
        .expect("error")
    };
    println!("{}", String::from_utf8_lossy(&output.stdout));
}