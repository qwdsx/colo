use std::{fs, path::PathBuf};

use crate::config::Config;

const COLOR_NAMES: [&str; 8] = [
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "white"
];

pub struct Template {
    pub name: String,
    pub file_ext: String,
    pub template: String
}

impl Template {
    pub fn write(self: &Self, config: &Config) {
        let path = PathBuf::from(format!("{}/{}{}{}",
            config.output_folder.display(),
            self.name,
            config.suffix,
            file_ext(&self.file_ext)
        ));
        match fs::write(&path, &self.template) {
            Ok(()) => { println!("Wrote to file: {path:#?}") },
            Err(e) => { println!("Could not write to file: {path:#?}: {e}") }
        }
    }
}

pub fn template_alacritty(colors: &Vec<String>) -> Template {
    let mut template_vec = Vec::<String>::new();

    template_vec.push("[colors.primary]".into());
    template_vec.push(format!("background = \"{}\"", colors[0]));
    template_vec.push(format!("foreground = \"{}\"", colors[7]));

    template_vec.push("[colors.normal]".into());
    for n in 0..8 {
        template_vec.push(format!("{} = \"{}\"", COLOR_NAMES[n], colors[n]));
    }

    template_vec.push("[colors.bright]".into());
    for n in 8..16 {
        template_vec.push(format!("{} = \"{}\"", COLOR_NAMES[n - 8], colors[n]));
    }

    Template {
        name: "alacritty".into(),
        file_ext: "toml".into(),
        template: template_vec.join("\n")
    }
}

pub fn template_xresources(colors: &Vec<String>) -> Template {
    let mut template_vec = Vec::<String>::new();

    template_vec.push(format!("#define BACKGROUND {}", colors[0]));
    template_vec.push(format!("#define FOREGROUND {}", colors[7]));

    for n in 0..8 {
        template_vec.push(format!("#define COLOR{} {}", n, colors[n]));
    }

    for n in 8..16 {
        template_vec.push(format!("#define COLOR{} {}", n, colors[n]));
    }

    Template {
        name: "xresources".into(),
        file_ext: "".into(),
        template: template_vec.join("\n")
    }
}

fn file_ext(file_ext: &str) -> String {
    if file_ext == "" {
        file_ext.into()
    } else {
        format!(".{}", file_ext)
    }
}