use std::path::PathBuf;
use clap::Parser;


#[derive(Parser)]
#[clap(about = "", version)]
pub struct Cli {
    #[arg(long, help = "Output folder for generated files.\nIf not set, generates a folder called 'output' in the current directory")]
    pub outputpath: Option<PathBuf>,

    #[arg(long, help = "Path to a file containing color codes.\nIf not set, assumes the file is in the current directory and is called 'colors.txt'")]
    pub colorpath: Option<PathBuf>,

    #[arg(long, help = "A string to be added to the end of filename. Default is no suffix")]
    pub suffix: Option<String>
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}