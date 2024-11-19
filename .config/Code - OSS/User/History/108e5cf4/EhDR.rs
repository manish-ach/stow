use std::process::Command;

fn main() {
    println!("hi");
    let output = {
        Command::new("ls")
        .arg("-a")
        .output()
    };

}
