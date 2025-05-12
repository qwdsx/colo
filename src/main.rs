
mod template;
use template::TemplateType;

mod cli;
mod config;

use config::Config;


fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::new();
    let config = Config::gen(cli.outputpath, cli.colorpath, cli.suffix)?;

    write_color_config(&config, TemplateType::Alacritty);
    write_color_config(&config, TemplateType::Xresources);
    write_color_config(&config, TemplateType::Polybar);

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
        },
        TemplateType::Polybar => {
            let template = template::template_polybar(&config.colors);
            template.write(&config);
        }
    }
}
