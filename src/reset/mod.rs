use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::{PathBuf};

pub mod windows;


// the thing that we'd append to the end of device id and user id
const ABRAKADABRA: &str = "abrakadabra12345";

fn append_to_file(file_path: &PathBuf, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(file_path.as_path())?;

    writeln!(&mut file, "{}", content)?;

    Ok(())
}

pub trait Resetter {
    fn reset_trial(&self) -> io::Result<()> {
        if let Ok(jetbrains_dir) = self.get_jetbrains_directory() {
            println!("JetBrains directory is: {:#?}", jetbrains_dir.as_path());

            let files = vec!["PermanentDeviceId", "PermanentUserId"];

            files.iter().for_each(|file| {
                let path: PathBuf = jetbrains_dir.join(file);
                if let Err(_) = append_to_file(&path, &ABRAKADABRA) {
                    println!("Couldn't write to file {file}. Aborting reset...");
                    std::process::exit(1);
                }
            });

            Ok(())
        } else {
            println!("Couldn't find JetBrains directory!");
            std::process::exit(1);
        }
    }

    fn get_jetbrains_directory(&self) -> io::Result<PathBuf>;
}