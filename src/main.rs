
mod cli;
mod error;
mod config;
use config::Config;

mod template;
use template::TemplateType;


fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::new();
    let config = Config::gen(cli.outputpath, cli.colorpath, cli.suffix)?;

    write_color_config(&config, TemplateType::Alacritty);
    write_color_config(&config, TemplateType::Xresources);
    write_color_config(&config, TemplateType::Polybar);

    Ok(())
}

fn write_color_config(config: &Config, template_type: TemplateType) {
    let template = template::get_template_from_type(&config.colors, template_type);
    template.write(&config);
}