mod preprocessor;
mod stdlib;
mod tests;
use std::{
    env::args,
    error::Error,
    fs::{read, write},
};

use preprocessor::process_single;

fn main() -> Result<(), Box<dyn Error>> {
    let argv = &args().into_iter().collect::<Vec<String>>();
    if argv.len() < 2 {
        return Err(Box::from("[On read] No file provided"));
    }
    let target =
        match read(&argv[1]) {
            Ok(v) => v,
            Err(_) => return Err(Box::from(
                "[On read] Insufficient permissions or file does not exist. Check your spelling",
            )),
        };
    let file = process_single(argv, target)?;
    write_files(vec![file])?;
    Ok(())
}

pub struct File<'a> {
    pub name: &'a str,
    pub contents: String,
}

pub fn write_files(files: Vec<File>) -> Result<(), Box<dyn Error>> {
    for file in files {
        write(format!("./{}.mcfunction", file.name), file.contents.trim())?;
    }
    Ok(())
}
