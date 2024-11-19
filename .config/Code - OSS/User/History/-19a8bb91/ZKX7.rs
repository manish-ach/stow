use std::{env, process::Command, io};
use dirs::home_dir;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 && args[0] == "wallpaper" {
        let img_path = "~/wallpapers/back.png";
        set_wallpaper(img_path)?;
    } else if  args.len() == 2 && args[1] =="-c" {
        configure()?;
    }   else {
        let img_path = "~/wallpapers/back.png";
        set_wallpaper(img_path)?;
        println!("Usage: wallpaper (to set wallpaper) or wallpaper -c (to interactively select wallpaper)");
    }

    Ok(())
}


fn configure() -> io::Result<()>{
    if let Some(home) = home_dir() {
        let wallpapers_path = home.join("wallpapers");
        let output = Command::new("ls")
            .arg(wallpapers_path.to_str().unwrap())
            .output()
            .expect("Failed to execute ls command");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    Ok(())
}


fn set_wallpaper(img_path: &str) -> io::Result<()> {

    println!("setting wallpaper to: {}", img_path);
    let path = "img ".to_owned() + img_path; 
    let _output = Command::new("swww")
        .arg(path)
        .output()
        .expect("Failed to execute swww command");
    Ok(())
}