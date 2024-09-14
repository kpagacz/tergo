use std::{
    ffi::OsStr,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{arg, Parser};
use tergo_lib::{config::Config, tergo_format};

#[derive(Parser, Debug)]
struct Cli {
    #[arg()]
    path: String,
}

#[derive(Debug)]
enum Error {
    OpenFile(std::io::Error),
    ReadFileToString(std::io::Error),
    SeekFile(std::io::Error),
    WriteToFile(std::io::Error),
    Formatting,
}

fn get_config() -> Config {
    Config::default()
}

fn format_file_in_place(path: &Path) -> Result<(), Error> {
    use Error::*;
    let mut file = std::fs::File::options()
        .read(true)
        .write(true)
        .open(path)
        .map_err(OpenFile)?;
    let mut content = String::default();
    file.read_to_string(&mut content)
        .map_err(ReadFileToString)?;
    let formatted = tergo_format(&content, Some(get_config())).map_err(|_| Formatting)?;
    file.seek(SeekFrom::Start(0)).map_err(SeekFile)?;
    file.write(formatted.as_bytes()).map_err(WriteToFile)?;
    Ok(())
}

fn list_r_files(path: &Path) -> Vec<PathBuf> {
    match path.read_dir() {
        Ok(paths) => {
            let mut r_files = vec![];
            for path in paths.flatten() {
                r_files.extend(list_r_files(&path.path()));
            }
            r_files
        }
        Err(_) => match path.extension() {
            Some(extension) => {
                if extension == OsStr::new("R") || extension == OsStr::new("r") {
                    vec![path.to_path_buf()]
                } else {
                    vec![]
                }
            }
            None => {
                vec![]
            }
        },
    }
}

fn format_r_files(path: &Path) {
    let r_files = list_r_files(path);
    for file in r_files {
        match format_file_in_place(&file) {
            Ok(_) => println!("Formatted: {:?}", &file),
            Err(e) => println!("Failed to format {:?}. Error: {e:?}", &file),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let path = PathBuf::from_str(&cli.path).unwrap();
    format_r_files(&path);
}
