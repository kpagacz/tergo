use std::{
    ffi::OsStr,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{arg, Parser};
use log::{info, trace};
use tergo_lib::{config::Config, tergo_format};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(default_value = ".")]
    path: String,

    #[arg(default_value = "tergo.toml")]
    config: String,
}

#[derive(Debug)]
enum Error {
    OpenFile(std::io::Error),
    ReadFileToString(std::io::Error),
    SeekFile(std::io::Error),
    WriteToFile(std::io::Error),
    Formatting,
}

fn get_config(path: &Path) -> Config {
    match std::fs::read_to_string(path) {
        Ok(config_file) => {
            let config: Config = toml::from_str(&config_file).unwrap_or_else(|_| Config::default());
            config
        }
        Err(_) => Config::default(),
    }
}

fn format_file_in_place(path: &Path, config: &Config) -> Result<(), Error> {
    use Error::*;
    let mut file = std::fs::File::options()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|e| {
            trace!("Error opening the file: {e:?}");
            OpenFile(e)
        })?;
    let mut content = String::default();
    file.read_to_string(&mut content).map_err(|e| {
        trace!("Error when reading the file: {e:?}");
        ReadFileToString(e)
    })?;
    let formatted = tergo_format(&content, Some(config)).map_err(|e| {
        trace!("Error when formatting: {e}");
        Formatting
    })?;
    file.seek(SeekFrom::Start(0)).map_err(WriteToFile)?;
    file.write(formatted.as_bytes()).map_err(|e| {
        trace!("Error writing to file: {e}");
        WriteToFile(e)
    })?;
    Ok(())
}

fn list_r_files(path: &Path) -> Vec<PathBuf> {
    trace!("List R files in a path: {path:?}");
    match path.read_dir() {
        Ok(paths) => {
            let mut r_files = vec![];
            for path in paths.flatten() {
                r_files.extend(list_r_files(&path.path()));
            }
            r_files
        }
        Err(_) => {
            trace!("{path:?} is not a directory");
            match path.extension() {
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
            }
        }
    }
}

fn format_r_files(path: &Path, config_path: &Path) {
    let r_files = list_r_files(path);
    let config = get_config(config_path);
    for file in r_files {
        match format_file_in_place(&file, &config) {
            Ok(_) => info!("Formatted: {:?}", &file),
            Err(e) => {
                info!("Failed to format {:?}. Error: {e:?}", &file);
                trace!("Error was: {e:?}");
            }
        }
    }
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let path = PathBuf::from_str(&cli.path).unwrap();
    let config_path = PathBuf::from_str(&cli.config).unwrap();
    format_r_files(&path, &config_path);
}
