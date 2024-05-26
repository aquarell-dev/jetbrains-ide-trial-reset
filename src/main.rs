use std::io::stdin;

use crate::reset::Resetter;
use crate::reset::windows::WindowsResetter;

mod reset;

fn reset(resetter: &impl Resetter) {
    match resetter.reset_trial() {
        Ok(()) => println!("Reset has been successful!"),
        Err(e) => println!("Error: {}", e)
    }
}

const GUIDE: &str = r#"First of all, currently it's not possible to reset only one specific IDE,
you either reset all of them or none at all.
After the reset would be done, you'd have to open up your IDE, you might be logged in,
but that's okay, you should log out and press the start trial button again and you're good to go."#;

fn main() {
    // todo implement some sorta menu or something
    println!("{GUIDE}");

    let mut input = String::new();

    println!("Press Enter to continue...");

    stdin().read_line(&mut input).expect("Pressed wrong button");

    match std::env::consts::OS {
        "windows" => reset(&WindowsResetter),
        _ => println!("Your OS is not supported just yet.")
    }

    println!("Press Enter to exit...");

    stdin().read_line(&mut input).expect("Pressed wrong button");
}
