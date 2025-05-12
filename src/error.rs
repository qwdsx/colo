use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColoError {
    #[error("could not find color palette file")]
    ColorPaletteError,
    #[error("could not write to file")]
    TemplateWriteError,
    #[error("default error")]
    Default,
}