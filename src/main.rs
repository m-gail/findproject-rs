use std::{
    env,
    error::Error,
    fs::{self, DirEntry},
    io::{self, ErrorKind},
    path::Path,
};

use clap::Parser;
use serde::{Deserialize, Serialize};

fn get_default_config_path() -> String {
    let full_config_path = "~/.config/findproject/config.yaml".to_string();
    let config_path = "findproject/config.yaml".to_string();
    let default_path = "~/.config".to_string();
    let xdg_config_home = env::var("XDG_CONFIG_HOME");

    let base_dir = match xdg_config_home {
        Ok(path) => path,
        Err(_) => default_path
    };

    return match Path::new(&base_dir).join(config_path).to_str() {
        Some(config_path) => config_path.to_string(),
        None => full_config_path,
    };
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = get_default_config_path())]
    config: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Directory {
    path: String,
    #[serde(default)]
    only_self: bool,
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(default)]
    sub_directories: Vec<Directory>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FindprojectConfig {
    directories: Vec<Directory>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config_path = expand_tilde(args.config);
    let yaml = fs::read_to_string(config_path)?;
    let config: FindprojectConfig = serde_yaml::from_str(&yaml)?;
    print_all_projects(config.directories);
    Ok(())
}

fn expand_tilde(string: String) -> String {
    let home_dir_result = env::var("HOME");

    return match home_dir_result {
        Ok(home_dir) => string.replace("~", &home_dir),
        Err(_) => string,
    };
}

fn print_all_projects(directories: Vec<Directory>) {
    let empty_string = String::from("");
    for directory in directories {
        let _ = walk_directory(directory, &empty_string);
    }
}

fn walk_directory(directory: Directory, base_path: &String) -> Result<(), Box<dyn Error>> {
    let full_path = match Path::new(base_path).join(&directory.path).to_str() {
        Some(string_path) => expand_tilde(String::from(string_path)),
        None => {
            return Err(Box::new(io::Error::new(
                ErrorKind::InvalidData,
                "Path could not be converted to String",
            )))
        }
    };

    if directory.only_self {
        println!("{}", full_path);
        return Ok(())
    }

    let sub_directories = fs::read_dir(&full_path)?;

    for sub_directory in sub_directories {
        let sub_directory = match sub_directory {
            Ok(dir_entry) => dir_entry,
            Err(_) => continue,
        };

        if should_print(&sub_directory, &directory) {
            println!("{}", sub_directory.path().display());
        }
    }

    for directory in directory.sub_directories {
        let _ = walk_directory(directory, &full_path);
    }

    Ok(())
}

fn should_print(dir_entry: &DirEntry, directory: &Directory) -> bool {
    let file_name = match dir_entry.file_name().to_str() {
        Some(name) => name.to_string(),
        None => return false,
    };

    return dir_entry.path().is_dir()
        && !directory.exclude.contains(&file_name)
        && !directory
            .sub_directories
            .iter()
            .find(|sub_directory| sub_directory.path == file_name)
            .is_some();
}
