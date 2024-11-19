use std::{env, process::Command, io};
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 && args[0] == "wallpaper" {
        let img_path = "~/wallpapers/back.png";
        set_wallpaper(img_path)?;
    } else if  args.len() == 2 && args[1] =="-c" {
        configure()?;
    }   else {
        let som = "ls ~/wallpapers/";
            let output = Command::new(som)
            .output()
            .expect("Failed to execute ls command");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("Usage: wallpaper (to set wallpaper) or wallpaper -c (to interactively select wallpaper)");
    }

    Ok(())
}


fn configure() -> io::Result<()>{
    let output = Command::new("ls")
    .arg("~/wallpapers/")
    .output()
    .expect("Failed to execute ls command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}


fn set_wallpaper(img_path: &str) -> io::Result<()> {

    let output = Command::new("ls")
    .arg("~/wallpapers/")
    .output()
    .expect("Failed to execute ls command");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    println!("setting wallpaper to: {}", img_path);

    Command::new("swww")
        .arg("img")
        .arg(img_path)
        .output()?;

    Command::new("w")
        .arg("-i")
        .arg(img_path)
        .output()?;
    Ok(())
}