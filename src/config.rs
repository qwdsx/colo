use std::{fs::{self, create_dir_all}, path::PathBuf};


pub struct Config {
    pub output_folder: PathBuf,
    pub colors: Vec<String>,
    pub suffix: String
}

impl Config {
    pub fn gen(
        output_folder: Option<PathBuf>,
        colorpath: Option<PathBuf>,
        suffix: Option<String>
    ) -> Result<Self, anyhow::Error> {
        let current_dir = std::env::current_dir()?;

        let output_folder = match output_folder {
            Some(s) => s,
            None => current_dir.join("colo_output")
        };

        if !output_folder.exists() {
            create_dir_all(&output_folder)?
        }

        let colorpath = match colorpath {
            Some(s) => s,
            None => current_dir.join("colors.txt")
        };

        let colorpath_string = match fs::read_to_string(&colorpath) {
            Ok(ok) => ok,
            Err(e) => panic!("Could not get colors file: {}", e)
        };

        let colors = get_colors(&colorpath_string);

        let suffix = match suffix {
            Some(s) => s,
            None => "".into()
        };

        Ok(
            Self {
                output_folder,
                colors,
                suffix
            }
        )
    }
}

fn get_colors(str: &str) -> Vec<String> {
    str.split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&f| !f.is_empty())
        .cloned()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
}