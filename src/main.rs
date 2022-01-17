use std::{thread, time::Duration};
use error_chain::error_chain;
use walkdir::WalkDir;
use rayon::prelude::*;

error_chain! {
    foreign_links {
        WalkDir(walkdir::Error);
        Io(std::io::Error);
        SystemTime(std::time::SystemTimeError);
    }
}

fn slow(i: i32) -> i32 {
    thread::sleep(Duration::from_millis(1000));
    i + 1
}

fn rayon_test() -> Option<String> {
    /*
    let a = WalkDir::new(".")
        .into_par_iter()
        .filter_map(|e| e.ok())
        .map(|&e| e.path().to_str().unwrap_or("").len());
    */
    let a: Vec<i32> = (0..30)
        .into_iter()
        .into_par_iter()
        .map(|i| slow(i))
        .collect();
    println!("{:?}", a);
    thread::sleep(Duration::from_millis(2000));
    None
}

fn main() -> Result<()> {
    let b = rayon_test();

    for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {

        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;

        if entry.path().is_file() {
            println!("file: {} {}",
                     entry.path().parent().unwrap().display(),
                     entry.file_name().to_str().unwrap());
        } else {
            println!("{}", entry.path().display());
        }
    }

    Ok(())
}

