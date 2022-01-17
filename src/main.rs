use std::{thread, time::Duration};
use error_chain::error_chain;
use walkdir::WalkDir;
use action::Action;

mod action;
mod walker;
#[cfg(test)]
mod tests;


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

/*
    thread::sleep(Duration::from_millis(2000));
*/

fn main() -> Result<()> {
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