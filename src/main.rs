use std::fs::{self};


mod template;
mod cli;
mod config;

use config::Config;

enum TemplateType {
    Alacritty,
    Xresources
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::new();
    let config = Config::gen(cli.outputpath, cli.colorpath, cli.suffix)?;

    write_color_config(&config, TemplateType::Alacritty);
    write_color_config(&config, TemplateType::Xresources);

    Ok(())
}

fn write_color_config(config: &Config, template_type: TemplateType) {
    match template_type {
        TemplateType::Alacritty => {
            let template = template::template_alacritty(&config.colors);
            template.write(&config);
        },
        TemplateType::Xresources => {
            let template = template::template_xresources(&config.colors);
            template.write(&config);
        }
    }
}
