use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{arg, Parser};
use log::{debug, info, trace, warn};
use tergo_lib::{tergo_format, Config};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(default_value = ".")]
    path: String,

    #[arg(default_value = "tergo.toml")]
    config: String,
}

#[derive(Debug)]
enum Error {
    ReadFileToString,
    WriteToFile,
    Formatting,
}

fn get_config(path: &Path) -> Config {
    match std::fs::read_to_string(path) {
        Ok(config_file) => {
            let config: Config = toml::from_str(&config_file).unwrap_or_else(|_| {
                warn!(
                    "Failed to deserialize the configuration file to Config. Using the default \
                     configuration."
                );
                Config::default()
            });
            config
        }
        Err(_) => {
            debug!("Configuration file not found. Using the default configuration.");
            Config::default()
        }
    }
}

fn format_file_in_place(path: &Path, config: &Config) -> Result<(), Error> {
    use Error::*;
    let content = std::fs::read_to_string(path).map_err(|e| {
        trace!("Error when reading the file {e}");
        ReadFileToString
    })?;
    let formatted = tergo_format(&content, Some(config)).map_err(|e| {
        trace!("Error when formatting: {e}");
        Formatting
    })?;
    trace!("Formatted code:\n:{}", formatted);
    std::fs::write(path, formatted).map_err(|e| {
        trace!("Error writing to file {e}");
        WriteToFile
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
    let ignored_paths: Vec<&Path> = config.exclusion_list.0.iter().map(Path::new).collect();
    debug!("Ignored paths: {ignored_paths:?}");
    for file in r_files {
        if ignored_paths
            .iter()
            .any(|&ignored_path| file.starts_with(ignored_path))
        {
            info!("Ignoring: {file:?}");
            continue;
        }
        debug!("Formatting: {file:?}");
        match format_file_in_place(&file, &config) {
            Ok(_) => info!("Formatted: {:?}", &file),
            Err(e) => {
                warn!("Failed to format {:?}. Error: {e:?}", &file);
                trace!("Error was: {e:?}");
            }
        }
    }
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    match simple_logger::init_with_env() {
        Ok(_) => {}
        Err(err) => println!("Failed to initialize logger: {:?}", err),
    }
    let cli = Cli::parse();

    let path = PathBuf::from_str(&cli.path).unwrap();
    let config_path = PathBuf::from_str(&cli.config).unwrap();
    format_r_files(&path, &config_path);
}
