use std::{env, process::Command};
use std::io::{self,Write};
use dirs::home_dir;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 && args[0] == "wallpaper" {
        configure()?;
    } else if  args.len() == 2 && args[1] =="-i" {
        let img = "kat.png";
        set_wallpaper(img)?;
    }   else {
        println!("Usage: wallpaper -i (to set wallpaper) or wallpaper (to interactively select wallpaper)");
    }

    Ok(())
}


fn configure() -> io::Result<()>{
    if let Some(home) = home_dir() {
        let wallpapers_path = home.join("wallpapers");
        let path = wallpapers_path.to_str().unwrap();
        let output = Command::new("ls")
            .arg(path)
            .output()
            .expect("Failed to execute ls command");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    print!(">> ");
    io::stdout().flush().unwrap();
    let mut inp = String::new();
    io::stdin().read_line(&mut  inp).unwrap();
    let wallpaper = inp.trim();

    if let Some(home) = home_dir() {
        let wallpapers_path = home.join("wallpapers");
        let image = wallpapers_path.join(wallpaper);
        let path = image.to_str().unwrap();

        let output = Command::new("swww")
            .arg("img")
            .arg(path)
            .output()
            .expect("Failed to execute swww command");

        let pywal = Command::new("wal")
            .arg("-i")
            .arg(path)
            .output()
            .expect("Failed to execute wal command");
    }
    
    Ok(())
}


fn set_wallpaper(img_path: &str) -> io::Result<()> {

    println!("setting wallpaper to: {}", img_path);
    if let Some(home) = home_dir() {
        let wallpapers_path = home.join("wallpapers");
        let image = wallpapers_path.join(img_path);
        let path = image.to_str().unwrap();

        let output = Command::new("swww")
            .arg("img")
            .arg(path)
            .output()
            .expect("Failed to execute swww command");

        let pywal = Command::new("wal")
            .arg("-i")
            .arg(path)
            .output()
            .expect("Failed to execute wal command");
    }
    Ok(())
}